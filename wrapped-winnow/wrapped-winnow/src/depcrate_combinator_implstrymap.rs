// Generated macro for TryMap (struct)
macro_rules! Depcrate_combinator_implsTryMap {
() => {
// Module: crate::combinator::impls
// Provides: {"TryMap"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::try_map`]"] pub struct TryMap < F , G , I , O , O2 , E , E2 > where F : Parser < I , O , E > , G : FnMut (O) -> Result < O2 , E2 > , I : Stream , E : FromExternalError < I , E2 > , E : ParserError < I > , { pub (crate) parser : F , pub (crate) map : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , pub (crate) e2 : core :: marker :: PhantomData < E2 > , }
};
}
