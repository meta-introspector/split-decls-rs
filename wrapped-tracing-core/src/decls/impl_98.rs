macro_rules! deps {
    () => {
        DefaultGuard!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Drop for DefaultGuard { # [inline] fn drop (& mut self) { let prev = CURRENT_STATE . try_with (| state | state . default . replace (self . 0 . take ())) ; SCOPED_COUNT . fetch_sub (1 , Ordering :: Release) ; drop (prev) } }
    };
}

impl_98!();