macro_rules! deps {
    () => {
        FileEncoder!();
    };
}

macro_rules! BUF_SIZE {
    () => {
        deps!();
        # [doc = " The size of the buffer in `FileEncoder`."] const BUF_SIZE : usize = 64 * 1024 ;
    };
}

BUF_SIZE!();