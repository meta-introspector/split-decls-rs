macro_rules! deps {
    () => {
        Origin!();
        Process!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Origin { # [doc = " Extracts the Origin from a raw `siginfo_t` structure."] # [doc = ""] # [doc = " This function is async-signal-safe, can be called inside a signal handler."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " On systems where the structure is backed by an union on the C side, this requires the"] # [doc = " `si_code` and `si_signo` fields must be set properly according to what fields are"] # [doc = " available."] # [doc = ""] # [doc = " The value passed by kernel satisfies this, care must be taken only when constructed"] # [doc = " manually."] pub unsafe fn extract (info : & siginfo_t) -> Self { let cause = sighook_signal_cause (info) ; let process = if cause . has_process () { let process = Process :: extract (info) ; if cfg ! (target_os = "macos") && process . pid == 0 && process . uid == 0 { None } else { Some (process) } } else { None } ; let signal = info . si_signo ; Origin { cause : cause . into () , signal , process , } } }
    };
}

impl_89!();