macro_rules! Result {
    () => {
        # [doc = " A specialized [`Result`] type for `rustix` APIs."] pub type Result < T > = result :: Result < T , Errno > ;
    };
}

Result!();