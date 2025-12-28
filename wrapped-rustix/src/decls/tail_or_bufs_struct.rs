macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! tail_or_bufs_struct {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] pub struct tail_or_bufs_struct { pub tail : UnionField < buf_ring_tail_struct > , pub bufs : UnionField < buf_ring_bufs_struct > , pub union_field : [u64 ; 2] , }
    };
}

tail_or_bufs_struct!()