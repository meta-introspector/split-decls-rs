macro_rules! io_sqring_offsets {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_sqring_offsets { pub head : u32 , pub tail : u32 , pub ring_mask : u32 , pub ring_entries : u32 , pub flags : u32 , pub dropped : u32 , pub array : u32 , # [doc (hidden)] pub resv1 : u32 , pub user_addr : io_uring_ptr , }
    };
}

io_sqring_offsets!();