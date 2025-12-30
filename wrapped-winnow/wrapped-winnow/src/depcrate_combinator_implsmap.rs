// Generated macro for Map (struct)
macro_rules! Depcrate_combinator_implsMap {
() => {
// Module: crate::combinator::impls
// Provides: {"Map"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::map`]"] pub struct Map < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> O2 , { pub (crate) parser : F , pub (crate) map : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
