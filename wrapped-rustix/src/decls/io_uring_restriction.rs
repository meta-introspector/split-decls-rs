macro_rules! deps {
    () => {
        IoringRestrictionOp!();
    };
}

macro_rules! io_uring_restriction {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone , Default)] # [non_exhaustive] pub struct io_uring_restriction { pub opcode : IoringRestrictionOp , pub register_or_sqe_op_or_sqe_flags : register_or_sqe_op_or_sqe_flags_union , # [doc (hidden)] pub resv : u8 , # [doc (hidden)] pub resv2 : [u32 ; 3] , }
    };
}

io_uring_restriction!()