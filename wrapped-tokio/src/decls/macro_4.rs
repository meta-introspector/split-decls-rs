macro_rules! macro_4 {
    () => {
        # [cfg (all (feature = "taskdump" , not (doc) , not (all (target_os = "linux" , any (target_arch = "aarch64" , target_arch = "x86" , target_arch = "x86_64")))))] compile_error ! ("The `taskdump` feature is only currently supported on \
linux, on `aarch64`, `x86` and `x86_64`.") ;
    };
}

macro_4!();