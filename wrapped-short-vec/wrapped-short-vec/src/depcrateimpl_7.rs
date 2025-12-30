// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl VisitError { fn into_de_error < 'de , A > (self) -> A :: Error where A : SeqAccess < 'de > , { match self { VisitError :: TooLong (len) => de :: Error :: invalid_length (len , & "three or fewer bytes") , VisitError :: TooShort (len) => de :: Error :: invalid_length (len , & "more bytes") , VisitError :: Overflow (val) => de :: Error :: invalid_value (de :: Unexpected :: Unsigned (val as u64) , & "a value in the range [0, 65535]" ,) , VisitError :: Alias => de :: Error :: invalid_value (de :: Unexpected :: Other ("alias encoding") , & "strict form encoding" ,) , VisitError :: ByteThreeContinues => de :: Error :: invalid_value (de :: Unexpected :: Other ("continue signal on byte-three") , & "a terminal signal on or before byte-three" ,) , } } }
};
}
