// Generated macro for RetryAfter (struct)
macro_rules! Depcrate_combinator_implsRetryAfter {
() => {
// Module: crate::combinator::impls
// Provides: {"RetryAfter"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::retry_after`]"] # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub struct RetryAfter < P , R , I , O , E > where P : Parser < I , O , E > , R : Parser < I , () , E > , I : Stream , I : Recover < E > , E : ParserError < I > + FromRecoverableError < I , E > , { pub (crate) parser : P , pub (crate) recover : R , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
