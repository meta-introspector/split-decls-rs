macro_rules! impl_ {
    () => {
        # [cfg (not (feature = "nightly"))] mod impl_ { pub use std :: collections :: { HashMap as SsoHashMap , HashSet as SsoHashSet } ; # [inline] pub fn ensure_sufficient_stack < R > (f : impl FnOnce () -> R) -> R { f () } }
    };
}

impl_!();