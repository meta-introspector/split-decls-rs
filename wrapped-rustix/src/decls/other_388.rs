macro_rules! other_388 {
    () => {
        # [doc = " User data in the io_uring API."] # [doc = ""] # [doc = " `io_uring`'s native API represents `user_data` fields as `u64` values. In"] # [doc = " order to preserve strict-provenance, use a union which allows users to"] # [doc = " optionally store pointers."] # [repr (C)] # [derive (Copy , Clone)] pub union io_uring_user_data { # [doc = " An arbitrary `u64`."] pub u64_ : u64 , # [doc = " A pointer."] pub ptr : io_uring_ptr , }
    };
}

other_388!()