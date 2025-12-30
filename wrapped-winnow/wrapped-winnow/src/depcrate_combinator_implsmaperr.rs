// Generated macro for MapErr (struct)
macro_rules! Depcrate_combinator_implsMapErr {
() => {
// Module: crate::combinator::impls
// Provides: {"MapErr"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::map_err`]"] pub struct MapErr < F , G , I , O , E , E2 > where F : Parser < I , O , E > , G : FnMut (E) -> E2 , { pub (crate) parser : F , pub (crate) map : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , pub (crate) e2 : core :: marker :: PhantomData < E2 > , }
};
}
