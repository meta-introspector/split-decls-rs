// Generated macro for SecpSignatureOffsets (struct)
macro_rules! DepcrateSecpSignatureOffsets {
() => {
// Module: crate
// Provides: {"SecpSignatureOffsets"}
// Dependencies: {}
# [doc = " Offsets of signature data within a secp256k1 instruction."] # [doc = ""] # [doc = " See the [module documentation][md] for a complete description."] # [doc = ""] # [doc = " [md]: self"] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , Eq , PartialEq)] pub struct SecpSignatureOffsets { # [doc = " Offset to 64-byte signature plus 1-byte recovery ID."] pub signature_offset : u16 , # [doc = " Within the transaction, the index of the instruction whose instruction data contains the signature."] pub signature_instruction_index : u8 , # [doc = " Offset to 20-byte Ethereum address."] pub eth_address_offset : u16 , # [doc = " Within the transaction, the index of the instruction whose instruction data contains the address."] pub eth_address_instruction_index : u8 , # [doc = " Offset to start of message data."] pub message_data_offset : u16 , # [doc = " Size of message data in bytes."] pub message_data_size : u16 , # [doc = " Within the transaction, the index of the instruction whose instruction data contains the message."] pub message_instruction_index : u8 , }
};
}
