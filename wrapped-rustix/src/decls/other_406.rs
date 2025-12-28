macro_rules! deps {
    () => {
        IoringMsgringCmds!();
    };
}

macro_rules! other_406 {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Copy , Clone)] pub union addr_or_splice_off_in_union { pub addr : io_uring_ptr , pub splice_off_in : u64 , pub msgring_cmd : IoringMsgringCmds , pub user_data : io_uring_user_data , }
    };
}

other_406!()