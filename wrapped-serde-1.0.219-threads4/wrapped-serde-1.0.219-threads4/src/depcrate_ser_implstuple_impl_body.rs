// Generated macro for tuple_impl_body (macro)
macro_rules! Depcrate_ser_implstuple_impl_body {
() => {
// Module: crate::ser::impls
// Provides: {"tuple_impl_body"}
// Dependencies: {}
macro_rules ! tuple_impl_body { ($ len : expr => ($ ($ n : tt) +)) => { # [inline] fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut tuple = tri ! (serializer . serialize_tuple ($ len)) ; $ (tri ! (tuple . serialize_element (& self .$ n)) ;) + tuple . end () } } ; }
};
}
