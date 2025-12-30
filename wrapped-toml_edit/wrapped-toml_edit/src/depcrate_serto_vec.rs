// Generated macro for to_vec (function)
macro_rules! Depcrate_serto_vec {
() => {
// Module: crate::ser
// Provides: {"to_vec"}
// Dependencies: {}
# [doc = " Serialize the given data structure as a TOML byte vector."] # [doc = ""] # [doc = " Serialization can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, if `T` contains a map with non-string keys, or if `T` attempts to"] # [doc = " serialize an unsupported datatype such as an enum, tuple, or tuple struct."] # [cfg (feature = "display")] pub fn to_vec < T > (value : & T) -> Result < Vec < u8 > , Error > where T : serde_core :: ser :: Serialize + ? Sized , { to_string (value) . map (| e | e . into_bytes ()) }
};
}
