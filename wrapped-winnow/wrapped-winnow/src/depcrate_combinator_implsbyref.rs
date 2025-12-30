// Generated macro for ByRef (struct)
macro_rules! Depcrate_combinator_implsByRef {
() => {
// Module: crate::combinator::impls
// Provides: {"ByRef"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::by_ref`]"] pub struct ByRef < 'p , P , I , O , E > { pub (crate) p : & 'p mut P , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
