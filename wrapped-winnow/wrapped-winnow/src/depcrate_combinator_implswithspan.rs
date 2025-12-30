// Generated macro for WithSpan (struct)
macro_rules! Depcrate_combinator_implsWithSpan {
() => {
// Module: crate::combinator::impls
// Provides: {"WithSpan"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::with_span`]"] pub struct WithSpan < F , I , O , E > where F : Parser < I , O , E > , I : Stream + Location , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
