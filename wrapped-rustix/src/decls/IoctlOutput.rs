macro_rules! IoctlOutput {
    () => {
        # [doc = " The type used by the `ioctl` to signify the output."] pub type IoctlOutput = c :: c_int ;
    };
}

IoctlOutput!();