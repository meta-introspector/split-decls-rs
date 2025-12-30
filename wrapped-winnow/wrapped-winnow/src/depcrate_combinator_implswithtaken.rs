// Generated macro for WithTaken (struct)
macro_rules! Depcrate_combinator_implsWithTaken {
() => {
// Module: crate::combinator::impls
// Provides: {"WithTaken"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::with_taken`]"] pub struct WithTaken < F , I , O , E > where F : Parser < I , O , E > , I : Stream , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
