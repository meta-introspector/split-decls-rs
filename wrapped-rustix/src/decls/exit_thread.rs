macro_rules! exit_thread {
    () => {
        # [doc = " `syscall(SYS_exit, status)`—Exit the current thread."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is a very low-level feature for implementing threading libraries."] # [inline] pub unsafe fn exit_thread (status : i32) -> ! { backend :: runtime :: syscalls :: tls :: exit_thread (status) }
    };
}

exit_thread!()