macro_rules! deps {
    () => {
        Guard!();
    };
}

macro_rules! with {
    () => {
        deps!();
        # [cfg (all (test , not (loom)))] pub (crate) fn with < R > (tid : usize , f : impl FnOnce () -> R) -> R { struct Guard (Option < usize >) ; impl Drop for Guard { fn drop (& mut self) { REGISTRATION . with (| r | r . 0 . set (self . 0 . take ())) ; } } let prev = REGISTRATION . with (| r | r . 0 . replace (Some (tid))) ; let _guard = Guard (prev) ; f () }
    };
}

with!();