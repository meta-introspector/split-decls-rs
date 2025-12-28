macro_rules! impl_39 {
    () => {
        impl Ord for Span { fn cmp (& self , rhs : & Self) -> Ordering { Ord :: cmp (& self . data () , & rhs . data ()) } }
    };
}

impl_39!()