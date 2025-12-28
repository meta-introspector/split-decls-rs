macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl Ord for Span { fn cmp (& self , rhs : & Self) -> Ordering { Ord :: cmp (& self . data () , & rhs . data ()) } }
    };
}

impl_263!()