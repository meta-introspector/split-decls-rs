macro_rules! deps {
    () => {
        IoringOp!();
    };
}

macro_rules! io_uring_probe_op {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_probe_op { pub op : IoringOp , # [doc (hidden)] pub resv : u8 , pub flags : IoringOpFlags , # [doc (hidden)] pub resv2 : u32 , }
    };
}

io_uring_probe_op!();