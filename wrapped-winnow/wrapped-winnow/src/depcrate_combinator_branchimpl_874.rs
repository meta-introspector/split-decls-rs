// Generated macro for impl_874 (impl)
macro_rules! Depcrate_combinator_branchimpl_874 {
() => {
// Module: crate::combinator::branch
// Provides: {"impl_874"}
// Dependencies: {}
impl < const N : usize , I : Stream , O , E : ParserError < I > , P : Parser < I , O , E > > Alt < I , O , E > for [P ; N] { fn choice (& mut self , input : & mut I) -> Result < O , E > { let mut error : Option < E > = None ; let start = input . checkpoint () ; for branch in self { input . reset (& start) ; match branch . parse_next (input) { Err (e) if e . is_backtrack () => { error = match error { Some (error) => Some (error . or (e)) , None => Some (e) , } ; } res => return res , } } match error { Some (e) => Err (e . append (input , & start)) , None => Err (ParserError :: assert (input , "`alt` needs at least one parser" ,)) , } } }
};
}
