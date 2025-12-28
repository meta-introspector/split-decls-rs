macro_rules! impl_394 {
    () => {
        # [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd for io_uring_user_data { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { unsafe { self . u64_ . partial_cmp (& other . u64_) } } }
    };
}

impl_394!();