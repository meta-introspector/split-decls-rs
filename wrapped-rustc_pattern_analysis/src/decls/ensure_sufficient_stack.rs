macro_rules! ensure_sufficient_stack {
    () => {
        # [cfg (not (feature = "rustc"))] pub fn ensure_sufficient_stack < R > (f : impl FnOnce () -> R) -> R { f () }
    };
}

ensure_sufficient_stack!()