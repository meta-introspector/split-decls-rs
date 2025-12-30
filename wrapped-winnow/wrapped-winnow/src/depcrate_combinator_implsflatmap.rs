// Generated macro for FlatMap (struct)
macro_rules! Depcrate_combinator_implsFlatMap {
() => {
// Module: crate::combinator::impls
// Provides: {"FlatMap"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::flat_map`]"] pub struct FlatMap < F , G , H , I , O , O2 , E > where F : Parser < I , O , E > , G : FnMut (O) -> H , H : Parser < I , O2 , E > , { pub (crate) f : F , pub (crate) g : G , pub (crate) h : core :: marker :: PhantomData < H > , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
