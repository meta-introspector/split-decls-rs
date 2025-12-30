// Generated macro for impl_761 (impl)
macro_rules! Depcrate_parsing_parsableimpl_761 {
() => {
// Module: crate::parsing::parsable
// Provides: {"impl_761"}
// Dependencies: {}
impl sealed :: Sealed for BorrowedFormatItem < '_ > { # [inline] fn parse_into < 'a > (& self , input : & 'a [u8] , parsed : & mut Parsed ,) -> Result < & 'a [u8] , error :: Parse > { Ok (parsed . parse_item (input , self) ?) } }
};
}
