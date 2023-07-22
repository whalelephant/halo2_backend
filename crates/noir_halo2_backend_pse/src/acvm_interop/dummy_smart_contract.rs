use crate::PseHalo2;
use acvm::SmartContract;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CWError {
    #[error("nope")]
    NotSupported,
}

impl SmartContract for PseHalo2 {
    type Error = CWError;

    /// Get ethereum verification contract from Verification Key
    fn eth_contract_from_vk(
        &self,
        mut _common_reference_string: &[u8],
        _verification_key: &[u8],
    ) -> Result<String, Self::Error> {
        Err(CWError::NotSupported)
    }
}
