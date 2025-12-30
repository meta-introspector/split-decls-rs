// Generated macro for impl_48 (impl)
macro_rules! Depcrate_algorithms_utilsimpl_48 {
() => {
// Module: crate::algorithms::utils
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'a , 'b , A , B > PartialEq < UniqueItem < 'a , A > > for UniqueItem < 'b , B > where A : Index < usize > + 'b + ? Sized , B : Index < usize > + 'b + ? Sized , B :: Output : PartialEq < A :: Output > , { # [inline (always)] fn eq (& self , other : & UniqueItem < 'a , A >) -> bool { self . value () == other . value () } }
};
}
