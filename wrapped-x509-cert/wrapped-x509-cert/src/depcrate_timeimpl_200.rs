// Generated macro for impl_200 (impl)
macro_rules! Depcrate_timeimpl_200 {
() => {
// Module: crate::time
// Provides: {"impl_200"}
// Dependencies: {}
impl < P : Profile > :: der :: EncodeValue for Validity < P > { fn value_len (& self) -> :: der :: Result < :: der :: Length > { [P :: time_encoding (self . not_before) ? . encoded_len () ? , P :: time_encoding (self . not_after) ? . encoded_len () ? ,] . into_iter () . try_fold (Length :: ZERO , | acc , len | acc + len) } fn encode_value (& self , writer : & mut impl :: der :: Writer) -> :: der :: Result < () > { P :: time_encoding (self . not_before) ? . encode (writer) ? ; P :: time_encoding (self . not_after) ? . encode (writer) ? ; Ok (()) } }
};
}
