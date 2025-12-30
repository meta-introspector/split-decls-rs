// Generated macro for impl_37 (impl)
macro_rules! Depcrateimpl_37 {
() => {
// Module: crate
// Provides: {"impl_37"}
// Dependencies: {}
impl < T > ops :: IndexMut < usize > for Slab < T > { # [track_caller] fn index_mut (& mut self , key : usize) -> & mut T { match self . entries . get_mut (key) { Some (& mut Entry :: Occupied (ref mut v)) => v , _ => panic ! ("invalid key") , } } }
};
}
