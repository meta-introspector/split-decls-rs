// Generated macro for impl_96 (impl)
macro_rules! Depcrate_setimpl_96 {
() => {
// Module: crate::set
// Provides: {"impl_96"}
// Dependencies: {}
impl < T : SetMember > Set < T > { pub const EMPTY : Self = Set (PhantomData , 0) ; pub fn contains (self , value : T) -> bool { (value . bit_mask () & self . 1) == value . bit_mask () } pub const fn iter (self) -> Iter < T > { Iter { index : 0 , set : self } } }
};
}
