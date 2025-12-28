macro_rules! IORING_REGISTER_FILES_SKIP {
    () => {
        # [doc = " `IORING_REGISTER_FILES_SKIP`"] pub const IORING_REGISTER_FILES_SKIP : BorrowedFd < 'static > = unsafe { BorrowedFd :: < 'static > :: borrow_raw (sys :: IORING_REGISTER_FILES_SKIP as RawFd) } ;
    };
}

IORING_REGISTER_FILES_SKIP!()