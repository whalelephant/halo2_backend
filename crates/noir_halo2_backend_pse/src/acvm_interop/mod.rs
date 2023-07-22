use crate::PseHalo2;
use acvm::Backend;

mod common_reference_string;
mod proof_system;
mod pwg;

#[cfg(feature = "cosmwasm")]
mod dummy_smart_contract;
#[cfg(not(feature = "cosmwasm"))]
mod smart_contract;

impl Backend for PseHalo2 {}
