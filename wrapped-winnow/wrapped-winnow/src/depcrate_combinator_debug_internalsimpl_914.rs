// Generated macro for impl_914 (impl)
macro_rules! Depcrate_combinator_debug_internalsimpl_914 {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"impl_914"}
// Dependencies: {}
impl < P , D , I , O , E > Parser < I , O , E > for Trace < P , D , I , O , E > where P : Parser < I , O , E > , I : Stream , D : std :: fmt :: Display , E : ParserError < I > , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { let depth = Depth :: new () ; let original = i . checkpoint () ; start (* depth , & self . name , self . call_count , i) ; let res = self . parser . parse_next (i) ; let consumed = i . offset_from (& original) ; let severity = Severity :: with_result (& res) ; end (* depth , & self . name , self . call_count , consumed , severity) ; self . call_count += 1 ; res } }
};
}
