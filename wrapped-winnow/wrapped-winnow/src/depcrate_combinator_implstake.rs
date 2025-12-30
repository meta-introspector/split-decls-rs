// Generated macro for Take (struct)
macro_rules! Depcrate_combinator_implsTake {
() => {
// Module: crate::combinator::impls
// Provides: {"Take"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::take`]"] pub struct Take < F , I , O , E > where F : Parser < I , O , E > , I : Stream , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
