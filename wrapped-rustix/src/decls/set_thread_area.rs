macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! set_thread_area {
    () => {
        deps!();
        # [cfg (target_arch = "x86")] # [inline] pub unsafe fn set_thread_area (u_info : & mut UserDesc) -> io :: Result < () > { backend :: runtime :: syscalls :: tls :: set_thread_area (u_info) }
    };
}

set_thread_area!()