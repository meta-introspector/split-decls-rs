// Generated macro for impl_1098 (impl)
macro_rules! Depcrate_combinator_implsimpl_1098 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1098"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < P , R , I , O , E > Parser < I , O , E > for RetryAfter < P , R , I , O , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < O , E > { if I :: is_recovery_supported () { retry_after_inner (& mut self . parser , & mut self . recover , i) } else { self . parser . parse_next (i) } } }
};
}
