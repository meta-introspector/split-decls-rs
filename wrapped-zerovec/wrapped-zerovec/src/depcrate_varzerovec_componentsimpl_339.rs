// Generated macro for impl_339 (impl)
macro_rules! Depcrate_varzerovec_componentsimpl_339 {
() => {
// Module: crate::varzerovec::components
// Provides: {"impl_339"}
// Dependencies: {}
impl < 'a , T , F > VarZeroVecComponents < 'a , T , F > where T : VarULE , T : ? Sized , T : Ord , F : VarZeroVecFormat , { # [doc = " Binary searches a sorted `VarZeroVecComponents<T>` for the given element. For more information, see"] # [doc = " the primitive function [`binary_search`](slice::binary_search)."] pub fn binary_search (& self , needle : & T) -> Result < usize , usize > { self . binary_search_by (| probe | probe . cmp (needle)) } pub fn binary_search_in_range (& self , needle : & T , range : Range < usize > ,) -> Option < Result < usize , usize > > { self . binary_search_in_range_by (| probe | probe . cmp (needle) , range) } }
};
}
