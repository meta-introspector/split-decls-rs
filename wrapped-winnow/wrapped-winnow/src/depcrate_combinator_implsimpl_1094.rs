// Generated macro for impl_1094 (impl)
macro_rules! Depcrate_combinator_implsimpl_1094 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1094"}
// Dependencies: {}
impl < P , I , O , E , F , C , FI > Parser < I , O , E > for ContextWith < P , I , O , E , F , C , FI > where P : Parser < I , O , E > , I : Stream , E : AddContext < I , C > , E : ParserError < I > , F : Fn () -> FI + Clone , C : core :: fmt :: Debug , FI : Iterator < Item = C > , { # [inline] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { let context = self . context . clone () ; let start = i . checkpoint () ; (self . parser) . parse_next (i) . map_err (| mut err | { for context in context () { err = err . add_context (i , & start , context) ; } err }) } }
};
}
