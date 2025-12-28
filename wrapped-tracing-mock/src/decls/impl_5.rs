macro_rules! deps {
    () => {
        HasAncestry!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl HasAncestry for & Attributes < '_ > { fn is_contextual (& self) -> bool { (self as & Attributes < '_ >) . is_contextual () } fn is_root (& self) -> bool { (self as & Attributes < '_ >) . is_root () } fn parent (& self) -> Option < & span :: Id > { (self as & Attributes < '_ >) . parent () } }
    };
}

impl_5!()