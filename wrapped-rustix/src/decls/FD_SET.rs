macro_rules! FD_SET {
    () => {
        # [doc = " wasi-libc's `fd_set` type. The libc bindings for it have private fields, so"] # [doc = " we redeclare it for ourselves so that we can access the fields. They're"] # [doc = " publicly exposed in wasi-libc."] # [cfg (target_os = "wasi")] # [repr (C)] struct FD_SET { # [doc = " The wasi-libc headers call this `__nfds`."] fd_count : usize , # [doc = " The wasi-libc headers call this `__fds`."] fd_array : [i32 ; c :: FD_SETSIZE] , }
    };
}

FD_SET!()