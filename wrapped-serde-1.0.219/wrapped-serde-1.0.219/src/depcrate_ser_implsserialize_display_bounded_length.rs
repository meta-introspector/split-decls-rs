// Generated macro for serialize_display_bounded_length (macro)
macro_rules! Depcrate_ser_implsserialize_display_bounded_length {
() => {
// Module: crate::ser::impls
// Provides: {"serialize_display_bounded_length"}
// Dependencies: {}
# [doc = " Serialize a value that implements `Display` as a string, when that string is"] # [doc = " statically known to never have more than a constant `MAX_LEN` bytes."] # [doc = ""] # [doc = " Panics if the `Display` impl tries to write more than `MAX_LEN` bytes."] # [cfg (any (feature = "std" , not (no_core_net)))] macro_rules ! serialize_display_bounded_length { ($ value : expr , $ max : expr , $ serializer : expr) => { { let mut buffer = [0u8 ; $ max] ; let mut writer = crate :: format :: Buf :: new (& mut buffer) ; write ! (& mut writer , "{}" , $ value) . unwrap () ; $ serializer . serialize_str (writer . as_str ()) } } ; }
};
}
