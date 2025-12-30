// Generated macro for impl_166 (impl)
macro_rules! Depcrate_parserimpl_166 {
() => {
// Module: crate::parser
// Provides: {"impl_166"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < I , O , E > Parser < I , O , E > for Box < dyn Parser < I , O , E > + '_ > { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { (* * self) . parse_next (i) } }
};
}
