macro_rules! RUNTIME_SHUTTING_DOWN_ERROR {
    () => {
        # [doc = " Error string explaining that the Tokio context is shutting down and cannot drive timers."] pub (crate) const RUNTIME_SHUTTING_DOWN_ERROR : & str = "A Tokio 1.x context was found, but it is being shutdown." ;
    };
}

RUNTIME_SHUTTING_DOWN_ERROR!();