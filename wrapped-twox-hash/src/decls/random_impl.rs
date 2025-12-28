macro_rules! deps {
    () => {
        State!();
        Hasher!();
    };
}

macro_rules! random_impl {
    () => {
        deps!();
        # [cfg (feature = "random")] # [cfg_attr (docsrs , doc (cfg (feature = "random")))] mod random_impl { use super :: * ; # [doc = " Constructs a randomized seed and reuses it for multiple hasher"] # [doc = " instances."] # [derive (Clone)] pub struct RandomState (State) ; impl Default for RandomState { fn default () -> Self { Self :: new () } } impl RandomState { fn new () -> Self { Self (State :: with_seed (rand :: random ())) } } impl BuildHasher for RandomState { type Hasher = Hasher ; fn build_hasher (& self) -> Self :: Hasher { self . 0 . build_hasher () } } # [cfg (test)] mod test { use std :: collections :: HashMap ; use super :: * ; const _TRAITS : () = { const fn is_clone < T : Clone > () { } is_clone :: < RandomState > () ; } ; # [test] fn can_be_used_in_a_hashmap_with_a_random_seed () { let mut hash : HashMap < _ , _ , RandomState > = Default :: default () ; hash . insert (42 , "the answer") ; assert_eq ! (hash . get (& 42) , Some (& "the answer")) ; } } }
    };
}

random_impl!()