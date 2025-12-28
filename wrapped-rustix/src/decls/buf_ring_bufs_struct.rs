macro_rules! deps {
    () => {
        IncompleteArrayField!();
    };
}

macro_rules! buf_ring_bufs_struct {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] pub struct buf_ring_bufs_struct { pub bufs : IncompleteArrayField < io_uring_buf > , }
    };
}

buf_ring_bufs_struct!()