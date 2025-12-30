// Generated macro for uses_durable_nonce (function)
macro_rules! Depcrateuses_durable_nonce {
() => {
// Module: crate
// Provides: {"uses_durable_nonce"}
// Dependencies: {}
# [doc = " Returns true if transaction begins with an advance nonce instruction."] pub fn uses_durable_nonce (tx : & Transaction) -> Option < & CompiledInstruction > { let message = tx . message () ; message . instructions . get (NONCED_TX_MARKER_IX_INDEX as usize) . filter (| instruction | { matches ! (message . account_keys . get (instruction . program_id_index as usize) , Some (program_id) if system_program :: check_id (program_id)) && is_advance_nonce_instruction_data (& instruction . data) }) }
};
}
