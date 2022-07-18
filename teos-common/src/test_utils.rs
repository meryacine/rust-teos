use std::convert::TryInto;

use hex::FromHex;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

use bitcoin::consensus;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::SecretKey;
use bitcoin::Txid;

use crate::appointment::{Appointment, Locator};
use crate::cryptography;
use crate::receipts::{AppointmentReceipt, RegistrationReceipt};
use crate::UserId;

pub static TXID_HEX: &str = "338bda693c4a26e0d41a01f7f2887aaf48bf0bdf93e6415c9110b29349349d3e";
pub static TX_HEX: &str =  "010000000001010000000000000000000000000000000000000000000000000000000000000000ffffffff54038e830a1b4d696e656420627920416e74506f6f6c373432c2005b005e7a0ae3fabe6d6d7841cd582ead8ea5dd8e3de1173cae6fcd2a53c7362ebb7fb6f815604fe07cbe0200000000000000ac0e060005f90000ffffffff04d9476026000000001976a91411dbe48cc6b617f9c6adaf4d9ed5f625b1c7cb5988ac0000000000000000266a24aa21a9ed7248c6efddd8d99bfddd7f499f0b915bffa8253003cc934df1ff14a81301e2340000000000000000266a24b9e11b6d7054937e13f39529d6ad7e685e9dd4efa426f247d5f5a5bed58cdddb2d0fa60100000000000000002b6a2952534b424c4f434b3a054a68aa5368740e8b3e3c67bce45619c2cfd07d4d4f0936a5612d2d0034fa0a0120000000000000000000000000000000000000000000000000000000000000000000000000";

pub fn get_random_int<T>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn get_random_user_id() -> UserId {
    let (_, pk) = cryptography::get_random_keypair();

    UserId(pk)
}

pub fn generate_random_appointment(dispute_txid: Option<&Txid>) -> Appointment {
    let dispute_txid = match dispute_txid {
        Some(l) => *l,
        None => {
            let prev_txid_bytes = cryptography::get_random_bytes(32);
            Txid::from_slice(&prev_txid_bytes).unwrap()
        }
    };

    let tx_bytes = Vec::from_hex(TX_HEX).unwrap();
    let penalty_tx = consensus::deserialize(&tx_bytes).unwrap();

    let mut raw_locator: [u8; 16] = cryptography::get_random_bytes(16).try_into().unwrap();
    raw_locator.copy_from_slice(&dispute_txid[..16]);
    let locator = Locator::from_slice(&raw_locator).unwrap();

    let encrypted_blob = cryptography::encrypt(&penalty_tx, &dispute_txid).unwrap();
    Appointment::new(locator, encrypted_blob, get_random_int())
}

pub fn get_random_registration_receipt() -> RegistrationReceipt {
    get_random_registration_receipt_with_expiry(get_random_int::<u32>() + 420)
}

pub fn get_random_registration_receipt_with_expiry(expiry: u32) -> RegistrationReceipt {
    let (sk, _) = cryptography::get_random_keypair();
    let mut receipt = RegistrationReceipt::new(
        get_random_user_id(),
        // Keep this number low, as it gets added frequenly in tests and could result in an overflow.
        get_random_int::<u32>() % 10_000_000,
        expiry - 420,
        expiry,
    );
    receipt.sign(&sk);

    receipt
}

pub fn get_random_appointment_receipt(tower_sk: SecretKey) -> AppointmentReceipt {
    let mut receipt = AppointmentReceipt::new("user_sig".into(), 42);
    receipt.sign(&tower_sk);

    receipt
}
