// Generated macro for MessageImprint (struct)
macro_rules! DepcrateMessageImprint {
() => {
// Module: crate
// Provides: {"MessageImprint"}
// Dependencies: {}
# [doc = " ```text"] # [doc = " MessageImprint ::= SEQUENCE  {"] # [doc = "    hashAlgorithm                AlgorithmIdentifier,"] # [doc = "    hashedMessage                OCTET STRING  }"] # [doc = " ```"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] pub struct MessageImprint { pub hash_algorithm : AlgorithmIdentifier < Any > , pub hashed_message : OctetString , }
};
}
