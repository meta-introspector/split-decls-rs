// Generated macro for NativeEndian (type)
macro_rules! Depcrate_byteorderNativeEndian {
() => {
// Module: crate::byteorder
// Provides: {"NativeEndian"}
// Dependencies: {}
# [doc = " The endianness used by this platform."] # [doc = ""] # [doc = " This is a type alias for [`BigEndian`] or [`LittleEndian`] depending on the"] # [doc = " endianness of the target platform."] # [cfg (target_endian = "little")] pub type NativeEndian = LittleEndian ;
};
}
