// Generated macro for impl_182 (impl)
macro_rules! Depcrateimpl_182 {
() => {
// Module: crate
// Provides: {"impl_182"}
// Dependencies: {}
# [doc = " URLs hash like their serialization."] impl hash :: Hash for Url { # [inline] fn hash < H > (& self , state : & mut H) where H : hash :: Hasher , { hash :: Hash :: hash (& self . serialization , state) } }
};
}
