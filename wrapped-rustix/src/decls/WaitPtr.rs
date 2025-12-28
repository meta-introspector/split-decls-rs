macro_rules! deps {
    () => {
        Wait!();
    };
}

macro_rules! WaitPtr {
    () => {
        deps!();
        # [doc = " A pointer in the [`Wait`] struct."] # [repr (C)] # [derive (Copy , Clone)] # [non_exhaustive] pub struct WaitPtr { # [cfg (all (target_pointer_width = "32" , target_endian = "big"))] # [doc (hidden)] pub __pad32 : u32 , # [cfg (all (target_pointer_width = "16" , target_endian = "big"))] # [doc (hidden)] pub __pad16 : u16 , # [doc = " The pointer value."] pub ptr : * mut c_void , # [cfg (all (target_pointer_width = "16" , target_endian = "little"))] # [doc (hidden)] pub __pad16 : u16 , # [cfg (all (target_pointer_width = "32" , target_endian = "little"))] # [doc (hidden)] pub __pad32 : u32 , }
    };
}

WaitPtr!();