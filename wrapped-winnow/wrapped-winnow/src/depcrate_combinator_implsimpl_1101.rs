// Generated macro for impl_1101 (impl)
macro_rules! Depcrate_combinator_implsimpl_1101 {
() => {
// Module: crate::combinator::impls
// Provides: {"impl_1101"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < P , R , I , O , E > Parser < I , Option < O > , E > for ResumeAfter < P , R , I , O , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { # [inline (always)] fn parse_next (& mut self , i : & mut I) -> Result < Option < O > , E > { if I :: is_recovery_supported () { resume_after_inner (& mut self . parser , & mut self . recover , i) } else { self . parser . parse_next (i) . map (Some) } } }
};
}
