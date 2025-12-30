// Generated macro for ResumeAfter (struct)
macro_rules! Depcrate_combinator_implsResumeAfter {
() => {
// Module: crate::combinator::impls
// Provides: {"ResumeAfter"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::resume_after`]"] # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub struct ResumeAfter < P , R , I , O , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { pub (crate) parser : P , pub (crate) recover : R , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
