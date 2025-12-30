// Generated macro for Span (struct)
macro_rules! Depcrate_combinator_implsSpan {
() => {
// Module: crate::combinator::impls
// Provides: {"Span"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::span`]"] pub struct Span < F , I , O , E > where F : Parser < I , O , E > , I : Stream + Location , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
