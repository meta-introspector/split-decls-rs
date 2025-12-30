// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 'a , 'b , D , F > Deserializer < 'a , 'b , D , F > where F : FnMut (Path) , { pub fn new (de : D , callback : & 'b mut F) -> Self { Deserializer { de , callback , path : Path :: Root , } } }
};
}
