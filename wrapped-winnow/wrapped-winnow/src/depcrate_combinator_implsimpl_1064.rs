// Generated macro for impl_1064 (impl)
macro_rules! Depcrate_combinator_implsimpl_1064 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1064"}
// Dependencies: {}
impl < F , G , I , O , O2 , E > Parser < I , O2 , E > for AndThen < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : Parser < O , O2 , E > , O : StreamIsPartial , I : Stream , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O2 , E > { let start = i . checkpoint () ; let mut o = self . outer . parse_next (i) ? ; let _ = o . complete () ; let o2 = self . inner . parse_next (& mut o) . map_err (| err | { i . reset (& start) ; err }) ? ; Ok (o2) } }
};
}
