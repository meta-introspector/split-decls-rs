macro_rules! io_uring_ptr {
    () => {
        # [doc = " A pointer in the io_uring API."] # [doc = ""] # [doc = " `io_uring`'s native API represents pointers as `u64` values. In order to"] # [doc = " preserve strict-provenance, use a `*mut c_void`. On platforms where"] # [doc = " pointers are narrower than 64 bits, this requires additional padding."] # [repr (C)] # [cfg_attr (any (target_arch = "arm" , target_arch = "powerpc") , repr (align (8)))] # [derive (Copy , Clone)] # [non_exhaustive] pub struct io_uring_ptr { # [cfg (all (target_pointer_width = "32" , target_endian = "big"))] # [doc (hidden)] pub __pad32 : u32 , # [cfg (all (target_pointer_width = "16" , target_endian = "big"))] # [doc (hidden)] pub __pad16 : u16 , # [doc = " The pointer value."] pub ptr : * mut c_void , # [cfg (all (target_pointer_width = "16" , target_endian = "little"))] # [doc (hidden)] pub __pad16 : u16 , # [cfg (all (target_pointer_width = "32" , target_endian = "little"))] # [doc (hidden)] pub __pad32 : u32 , }
    };
}

io_uring_ptr!()