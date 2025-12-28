macro_rules! SixtyFourBitPointer {
    () => {
        # [repr (C)] # [derive (Copy , Clone)] struct SixtyFourBitPointer { # [cfg (target_endian = "big")] # [cfg (target_pointer_width = "32")] _padding : u32 , pointer : * mut c_void , # [cfg (target_endian = "little")] # [cfg (target_pointer_width = "32")] _padding : u32 , }
    };
}

SixtyFourBitPointer!()