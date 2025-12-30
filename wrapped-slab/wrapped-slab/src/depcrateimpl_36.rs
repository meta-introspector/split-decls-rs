// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < T > ops :: Index < usize > for Slab < T > { type Output = T ; # [track_caller] fn index (& self , key : usize) -> & T { match self . entries . get (key) { Some (Entry :: Occupied (v)) => v , _ => panic ! ("invalid key") , } } }
};
}
