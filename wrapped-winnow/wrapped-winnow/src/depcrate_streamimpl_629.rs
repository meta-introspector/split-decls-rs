// Generated macro for impl_629 (impl)
macro_rules! Depcrate_streamimpl_629 {
() => {
// Module: crate::stream
// Provides: {"impl_629"}
// Dependencies: {}
# [cfg (feature = "std")] impl < K , S > Accumulate < K > for HashSet < K , S > where K : core :: cmp :: Eq + core :: hash :: Hash , S : BuildHasher + Default , { # [inline (always)] fn initial (capacity : Option < usize >) -> Self { let h = S :: default () ; match capacity { Some (capacity) => HashSet :: with_capacity_and_hasher (clamp_capacity :: < K > (capacity) , h) , None => HashSet :: with_hasher (h) , } } # [inline (always)] fn accumulate (& mut self , key : K) { self . insert (key) ; } }
};
}
