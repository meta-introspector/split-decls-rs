// Generated macro for Secp256r1SignatureOffsets (struct)
macro_rules! DepcrateSecp256r1SignatureOffsets {
() => {
// Module: crate
// Provides: {"Secp256r1SignatureOffsets"}
// Dependencies: {}
# [derive (Default , Debug , Copy , Clone , Zeroable , Pod , Eq , PartialEq)] # [repr (C)] pub struct Secp256r1SignatureOffsets { # [doc = " Offset to compact secp256r1 signature of 64 bytes"] pub signature_offset : u16 , # [doc = " Instruction index where the signature can be found"] pub signature_instruction_index : u16 , # [doc = " Offset to compressed public key of 33 bytes"] pub public_key_offset : u16 , # [doc = " Instruction index where the public key can be found"] pub public_key_instruction_index : u16 , # [doc = " Offset to the start of message data"] pub message_data_offset : u16 , # [doc = " Size of message data in bytes"] pub message_data_size : u16 , # [doc = " Instruction index where the message data can be found"] pub message_instruction_index : u16 , }
};
}
