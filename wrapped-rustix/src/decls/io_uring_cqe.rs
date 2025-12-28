macro_rules! deps {
    () => {
        IncompleteArrayField!();
    };
}

macro_rules! io_uring_cqe {
    () => {
        deps!();
        # [doc = " An io_uring Completion Queue Entry."] # [doc = ""] # [doc = " This does not derive `Copy` or `Clone` because the `big_cqe` field is not"] # [doc = " automatically copyable."] # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] pub struct io_uring_cqe { pub user_data : io_uring_user_data , pub res : i32 , pub flags : IoringCqeFlags , pub big_cqe : IncompleteArrayField < u64 > , }
    };
}

io_uring_cqe!();