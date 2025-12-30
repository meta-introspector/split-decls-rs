// Generated macro for impl_875 (impl)
macro_rules! Depcrate_combinator_branchimpl_875 {
() => {
// Module: crate::combinator::branch
// Provides: {"impl_875"}
// Dependencies: {}
impl < I : Stream , O , E : ParserError < I > , P : Parser < I , O , E > > Alt < I , O , E > for & mut [P] { fn choice (& mut self , input : & mut I) -> Result < O , E > { let mut error : Option < E > = None ; let start = input . checkpoint () ; for branch in self . iter_mut () { input . reset (& start) ; match branch . parse_next (input) { Err (e) if e . is_backtrack () => { error = match error { Some (error) => Some (error . or (e)) , None => Some (e) , } ; } res => return res , } } match error { Some (e) => Err (e . append (input , & start)) , None => Err (ParserError :: assert (input , "`alt` needs at least one parser" ,)) , } } }
};
}
