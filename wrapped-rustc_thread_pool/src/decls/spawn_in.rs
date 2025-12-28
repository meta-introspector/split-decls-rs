macro_rules! deps {
    () => {
        Registry!();
        AbortIfPanic!();
    };
}

macro_rules! spawn_in {
    () => {
        deps!();
        # [doc = " Spawns an asynchronous job in `registry.`"] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn spawn_in < F > (func : F , registry : & Arc < Registry >) where F : FnOnce () + Send + 'static , { let abort_guard = unwind :: AbortIfPanic ; let job_ref = unsafe { spawn_job (func , registry) } ; registry . inject_or_push (job_ref) ; mem :: forget (abort_guard) ; }
    };
}

spawn_in!();