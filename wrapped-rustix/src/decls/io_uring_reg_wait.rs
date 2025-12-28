macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! io_uring_reg_wait {
    () => {
        deps!();
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Default)] # [non_exhaustive] pub struct io_uring_reg_wait { pub ts : Timespec , pub min_wait_usec : u32 , pub flags : u32 , pub sigmask : io_uring_ptr , pub sigmask_sz : u32 , # [doc (hidden)] pub pad : [u32 ; 3] , # [doc (hidden)] pub pad2 : [u64 ; 2] , }
    };
}

io_uring_reg_wait!();