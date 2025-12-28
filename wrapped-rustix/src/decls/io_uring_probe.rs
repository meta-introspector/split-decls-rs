macro_rules! deps {
    () => {
        IoringOp!();
        IncompleteArrayField!();
    };
}

macro_rules! io_uring_probe {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] # [non_exhaustive] pub struct io_uring_probe { pub last_op : IoringOp , pub ops_len : u8 , # [doc (hidden)] pub resv : u16 , # [doc (hidden)] pub resv2 : [u32 ; 3] , pub ops : IncompleteArrayField < io_uring_probe_op > , }
    };
}

io_uring_probe!();