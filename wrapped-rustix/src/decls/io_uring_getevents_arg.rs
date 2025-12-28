macro_rules! io_uring_getevents_arg {
    () => {
        # [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct io_uring_getevents_arg { pub sigmask : io_uring_ptr , pub sigmask_sz : u32 , pub min_wait_usec : u32 , pub ts : io_uring_ptr , }
    };
}

io_uring_getevents_arg!()