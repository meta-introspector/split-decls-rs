macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl PartialOrd for Span { fn partial_cmp (& self , rhs : & Self) -> Option < Ordering > { PartialOrd :: partial_cmp (& self . data () , & rhs . data ()) } }
    };
}

impl_262!();