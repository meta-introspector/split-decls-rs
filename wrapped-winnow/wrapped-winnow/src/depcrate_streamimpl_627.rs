// Generated macro for impl_627 (impl)
macro_rules! Depcrate_streamimpl_627 {
() => {
// Module: crate::stream
// Provides: {"impl_627"}
// Dependencies: {}
# [cfg (feature = "std")] impl < K , V , S > Accumulate < (K , V) > for HashMap < K , V , S > where K : core :: cmp :: Eq + core :: hash :: Hash , S : BuildHasher + Default , { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { let h = S :: default () ; match capacity { Some (capacity) => { HashMap :: with_capacity_and_hasher (clamp_capacity :: < (K , V) > (capacity) , h) } None => HashMap :: with_hasher (h) , } } # [inline (always)] fn accumulate (& mut self , (key , value) : (K , V)) { self . insert (key , value) ; } }
};
}
