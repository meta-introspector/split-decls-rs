macro_rules! impl_38 {
    () => {
        impl PartialOrd for Span { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& self . data () , & rhs . data ()) } }
    };
}

impl_38!()