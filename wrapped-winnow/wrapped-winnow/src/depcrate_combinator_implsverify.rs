// Generated macro for Verify (struct)
macro_rules! Depcrate_combinator_implsVerify {
() => {
// Module: crate::combinator::impls
// Provides: {"Verify"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::verify`]"] pub struct Verify < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (& O2) -> bool , I : Stream , O : Borrow < O2 > , O2 : ? Sized , E : ParserError < I > , { pub (crate) parser : F , pub (crate) filter : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
