// Generated macro for impl_91 (impl)
macro_rules! Depcrate_deimpl_91 {
() => {
// Module: crate::de
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de : 'a , 'a > Deserialize < 'de > for Cow < 'a , Bytes > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let cow : Cow < [u8] > = Deserialize :: deserialize (deserializer) ? ; match cow { Cow :: Borrowed (bytes) => Ok (Cow :: Borrowed (Bytes :: new (bytes))) , Cow :: Owned (bytes) => Ok (Cow :: Owned (ByteBuf :: from (bytes))) , } } }
};
}
