// Generated macro for impl_765 (impl)
macro_rules! Depcrate_parsing_parsableimpl_765 {
() => {
// Module: crate::parsing::parsable
// Provides: {"impl_765"}
// Dependencies: {}
impl < T > sealed :: Sealed for T where T : Deref < Target : sealed :: Sealed > , { # [inline] fn parse_into < 'a > (& self , input : & 'a [u8] , parsed : & mut Parsed ,) -> Result < & 'a [u8] , error :: Parse > { self . deref () . parse_into (input , parsed) } }
};
}
