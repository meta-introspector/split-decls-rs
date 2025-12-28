macro_rules! raise {
    () => {
        # [doc = " The usual raise, just the safe wrapper around it."] # [doc = ""] # [doc = " This is async-signal-safe."] pub fn raise (sig : c_int) -> Result < () , Error > { let result = unsafe { libc :: raise (sig) } ; if result == - 1 { Err (Error :: last_os_error ()) } else { Ok (()) } }
    };
}

raise!();