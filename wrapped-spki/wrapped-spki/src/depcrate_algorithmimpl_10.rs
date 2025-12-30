// Generated macro for impl_10 (impl)
macro_rules! Depcrate_algorithmimpl_10 {
() => {
// Module: crate::algorithm
// Provides: {"impl_10"}
// Dependencies: {}
impl < Params > EncodeValue for AlgorithmIdentifier < Params > where Params : Encode , { fn value_len (& self) -> der :: Result < Length > { self . oid . encoded_len () ? + self . parameters . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . oid . encode (writer) ? ; self . parameters . encode (writer) ? ; Ok (()) } }
};
}
