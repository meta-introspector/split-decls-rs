// Generated macro for VerifyMap (struct)
macro_rules! Depcrate_combinator_implsVerifyMap {
() => {
// Module: crate::combinator::impls
// Provides: {"VerifyMap"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::verify_map`]"] pub struct VerifyMap < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> Option < O2 > , I : Stream , E : ParserError < I > , { pub (crate) parser : F , pub (crate) map : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
