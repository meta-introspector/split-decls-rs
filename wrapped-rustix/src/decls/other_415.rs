macro_rules! deps {
    () => {
        IoringOp!();
        IoringRegisterOp!();
    };
}

macro_rules! other_415 {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] pub union register_or_sqe_op_or_sqe_flags_union { pub register_op : IoringRegisterOp , pub sqe_op : IoringOp , pub sqe_flags : IoringSqeFlags , }
    };
}

other_415!()