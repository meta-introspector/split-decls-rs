// Generated macro for impl_7 (impl)
macro_rules! Depcrate_arrayimpl_7 {
() => {
// Module: crate::array
// Provides: {"impl_7"}
// Dependencies: {}
impl < T , S , const LANES : usize > Strategy for UniformArrayStrategy < S , [T ; LANES] > where T : core :: fmt :: Debug , S : Strategy < Value = T > , { type Tree = ArrayValueTree < [S :: Tree ; LANES] > ; type Value = [T ; LANES] ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let tree : [S :: Tree ; LANES] = unsafe { # [allow (clippy :: uninit_assumed_init)] let mut tree : [MaybeUninit < S :: Tree > ; LANES] = MaybeUninit :: uninit () . assume_init () ; for t in tree . iter_mut () { * t = MaybeUninit :: new (self . strategy . new_tree (runner) ?) } core :: mem :: transmute_copy (& tree) } ; Ok (ArrayValueTree { tree , shrinker : 0 , last_shrinker : None , }) } }
};
}
