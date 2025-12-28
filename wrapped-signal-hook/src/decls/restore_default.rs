macro_rules! restore_default {
    () => {
        # [cfg (windows)] fn restore_default (signal : c_int) -> Result < () , Error > { unsafe { if libc :: signal (signal , 0) == 0 { Ok (()) } else { Err (Error :: last_os_error ()) } } }
    };
}

restore_default!();