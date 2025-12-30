// Generated macro for impl_401 (impl)
macro_rules! Depcrate_ser_implsimpl_401 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_401"}
// Dependencies: {}
impl < T > Serialize for RefCell < T > where T : ? Sized + Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self . try_borrow () { Ok (value) => value . serialize (serializer) , Err (_) => Err (S :: Error :: custom ("already mutably borrowed")) , } } }
};
}
