// Generated macro for impl_82 (impl)
macro_rules! Depcrateimpl_82 {
() => {
// Module: crate
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'e , 'a > Expecting < 'e , 'a > { fn new (formatter : & 'e mut fmt :: Formatter < 'a >) -> Self { Expecting { formatter , count : 0 , last : None , } } fn push (& mut self , article : & str , item : & 'e str) -> fmt :: Result { self . count += 1 ; if self . count == 1 { if ! article . is_empty () { self . formatter . write_str (article) ? ; self . formatter . write_str (" ") ? ; } self . formatter . write_str (item) ? ; } else { if let Some (last) = self . last . take () { self . formatter . write_str (", ") ? ; self . formatter . write_str (last) ? ; } self . last = Some (item) ; } Ok (()) } fn flush (& mut self) -> fmt :: Result { if self . count == 0 { self . formatter . write_str ("unspecified") } else if let Some (last) = self . last . take () { self . formatter . write_str (" or ") ? ; self . formatter . write_str (last) } else { Ok (()) } } }
};
}
