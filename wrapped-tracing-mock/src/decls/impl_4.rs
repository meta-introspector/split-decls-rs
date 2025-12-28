macro_rules! deps {
    () => {
        HasAncestry!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl HasAncestry for & Event < '_ > { fn is_contextual (& self) -> bool { (self as & Event < '_ >) . is_contextual () } fn is_root (& self) -> bool { (self as & Event < '_ >) . is_root () } fn parent (& self) -> Option < & span :: Id > { (self as & Event < '_ >) . parent () } }
    };
}

impl_4!();