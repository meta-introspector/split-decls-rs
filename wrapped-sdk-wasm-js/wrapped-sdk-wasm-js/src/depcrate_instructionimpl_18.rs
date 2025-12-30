// Generated macro for impl_18 (impl)
macro_rules! Depcrate_instructionimpl_18 {
() => {
// Module: crate::instruction
// Provides: {"impl_18"}
// Dependencies: {}
# [wasm_bindgen] impl Instruction { # [doc = " Create a new `Instruction`"] # [wasm_bindgen (constructor)] pub fn constructor (program_id : Address) -> Self { solana_instruction :: Instruction :: new_with_bytes (program_id . inner , & [] , std :: vec :: Vec :: new ()) . into () } pub fn setData (& mut self , data : Uint8Array) -> Result < () , JsValue > { if data . length () as usize > MAX_INSTRUCTION_DATA_LEN { return Err (std :: format ! ("Instruction data too large: {} > {}" , data . length () , MAX_INSTRUCTION_DATA_LEN) . into ()) ; } self . inner . data = data . to_vec () ; Ok (()) } pub fn addAccount (& mut self , account_meta : AccountMeta) { self . inner . accounts . push (account_meta . inner) ; } }
};
}
