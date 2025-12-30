// Generated macro for impl_902 (impl)
macro_rules! Depcrate_combinator_coreimpl_902 {
() => {
// Module: crate::combinator::core
// Provides: {"impl_902"}
// Dependencies: {}
impl < F , I , O , E > core :: iter :: Iterator for & mut ParserIterator < F , I , O , E > where F : Parser < I , O , E > , I : Stream , E : ParserError < I > , { type Item = O ; fn next (& mut self) -> Option < Self :: Item > { if matches ! (self . state , State :: Running) { let start = self . input . checkpoint () ; match self . parser . parse_next (& mut self . input) { Ok (o) => { self . state = State :: Running ; Some (o) } Err (e) if e . is_backtrack () => { self . input . reset (& start) ; self . state = State :: Done ; None } Err (e) => { self . state = State :: Cut (e) ; None } } } else { None } } }
};
}
