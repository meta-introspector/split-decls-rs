// Generated macro for impl_763 (impl)
macro_rules! Depcrate_parsing_parsableimpl_763 {
() => {
// Module: crate::parsing::parsable
// Provides: {"impl_763"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl sealed :: Sealed for OwnedFormatItem { # [inline] fn parse_into < 'a > (& self , input : & 'a [u8] , parsed : & mut Parsed ,) -> Result < & 'a [u8] , error :: Parse > { Ok (parsed . parse_item (input , self) ?) } }
};
}
