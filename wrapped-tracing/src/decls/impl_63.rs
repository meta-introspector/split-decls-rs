macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl cmp :: PartialEq for Span { fn eq (& self , other : & Self) -> bool { match (& self . meta , & other . meta) { (Some (this) , Some (that)) => { this . callsite () == that . callsite () && self . inner == other . inner } _ => false , } } }
    };
}

impl_63!();