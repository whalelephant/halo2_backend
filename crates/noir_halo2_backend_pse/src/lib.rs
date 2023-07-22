mod acvm_interop;
mod dimension_measure;

mod assigned_map;
pub mod circuit_translator;
mod constrains;
mod halo2_params;
pub mod halo2_plonk_api;
mod tests;
#[cfg(all(target_family = "wasm", not(feature = "cosmwasm")))]
mod wasm;

#[derive(Debug, Clone)]
pub struct PseHalo2;

impl PseHalo2 {
    pub(crate) fn new() -> PseHalo2 {
        PseHalo2 {}
    }
}

impl Default for PseHalo2 {
    fn default() -> PseHalo2 {
        PseHalo2::new()
    }
}
