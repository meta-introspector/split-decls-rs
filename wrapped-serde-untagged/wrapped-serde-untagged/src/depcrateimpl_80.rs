// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , 'de , V , T > Visitor < 'de > for DefaultVisitor < 'a , V , T > where V : Expected , { type Value = T ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { self . expected . fmt (formatter) } }
};
}
