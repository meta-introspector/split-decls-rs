macro_rules! deps {
    () => {
        LittleEndian!();
        BigEndian!();
    };
}

macro_rules! NativeEndian {
    () => {
        deps!();
        # [doc = " The endianness used by this platform."] # [doc = ""] # [doc = " This is a type alias for [`BigEndian`] or [`LittleEndian`] depending on the"] # [doc = " endianness of the target platform."] # [cfg (target_endian = "little")] pub type NativeEndian = LittleEndian ;
    };
}

NativeEndian!()