macro_rules! buf_ring_tail_struct {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct buf_ring_tail_struct { # [doc (hidden)] pub resv1 : u64 , # [doc (hidden)] pub resv2 : u32 , # [doc (hidden)] pub resv3 : u16 , pub tail : u16 , }
    };
}

buf_ring_tail_struct!();