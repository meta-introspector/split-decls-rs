macro_rules! impl_382 {
    () => {
        # [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd for io_uring_ptr { # [inline] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . ptr . partial_cmp (& other . ptr) } }
    };
}

impl_382!()