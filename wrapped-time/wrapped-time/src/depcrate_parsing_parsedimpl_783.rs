// Generated macro for impl_783 (impl)
macro_rules! Depcrate_parsing_parsedimpl_783 {
() => {
// Module: crate::parsing::parsed
// Provides: {"impl_783"}
// Dependencies: {}
impl sealed :: AnyFormatItem for BorrowedFormatItem < '_ > { # [inline] fn parse_item < 'a > (& self , parsed : & mut Parsed , input : & 'a [u8] ,) -> Result < & 'a [u8] , error :: ParseFromDescription > { match self { Self :: Literal (literal) => Parsed :: parse_literal (input , literal) , Self :: Component (component) => parsed . parse_component (input , * component) , Self :: Compound (compound) => parsed . parse_items (input , compound) , Self :: Optional (item) => parsed . parse_item (input , * item) . or (Ok (input)) , Self :: First (items) => { let mut first_err = None ; for item in items . iter () { match parsed . parse_item (input , item) { Ok (remaining_input) => return Ok (remaining_input) , Err (err) if first_err . is_none () => first_err = Some (err) , Err (_) => { } } } match first_err { Some (err) => Err (err) , None => Ok (input) , } } } } }
};
}
