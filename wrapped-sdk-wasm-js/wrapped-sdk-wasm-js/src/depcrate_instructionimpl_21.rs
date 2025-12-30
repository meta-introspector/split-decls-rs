// Generated macro for impl_21 (impl)
macro_rules! Depcrate_instructionimpl_21 {
() => {
// Module: crate::instruction
// Provides: {"impl_21"}
// Dependencies: {}
# [wasm_bindgen] impl AccountMeta { # [doc = " Create a new writable `AccountMeta`"] pub fn newWritable (address : Address , is_signer : bool) -> Self { solana_instruction :: AccountMeta :: new (address . inner , is_signer) . into () } # [doc = " Create a new readonly `AccountMeta`"] pub fn newReadonly (address : Address , is_signer : bool) -> Self { solana_instruction :: AccountMeta :: new_readonly (address . inner , is_signer) . into () } }
};
}
