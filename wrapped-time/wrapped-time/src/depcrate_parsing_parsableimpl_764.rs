// Generated macro for impl_764 (impl)
macro_rules! Depcrate_parsing_parsableimpl_764 {
() => {
// Module: crate::parsing::parsable
// Provides: {"impl_764"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl sealed :: Sealed for [OwnedFormatItem] { # [inline] fn parse_into < 'a > (& self , input : & 'a [u8] , parsed : & mut Parsed ,) -> Result < & 'a [u8] , error :: Parse > { Ok (parsed . parse_items (input , self) ?) } }
};
}
