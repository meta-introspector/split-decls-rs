// Generated macro for impl_1092 (impl)
macro_rules! Depcrate_combinator_implsimpl_1092 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1092"}
// Dependencies: {}
impl < F , I , O , E , C > Parser < I , O , E > for Context < F , I , O , E , C > where F : Parser < I , O , E > , I : Stream , E : AddContext < I , C > , E : ParserError < I > , C : Clone + core :: fmt :: Debug , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { let context = self . context . clone () ; trace (DisplayDebug (self . context . clone ()) , move | i : & mut I | { let start = i . checkpoint () ; (self . parser) . parse_next (i) . map_err (| err | err . add_context (i , & start , context . clone ())) }) . parse_next (i) } }
};
}
