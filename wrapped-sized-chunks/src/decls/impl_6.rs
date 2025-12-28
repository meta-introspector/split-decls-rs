macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a , A , T > Iterator for Drain < 'a , A , T > { type Item = A ; fn next (& mut self) -> Option < Self :: Item > { self . array . remove (0) } fn size_hint (& self) -> (usize , Option < usize >) { (self . array . len () , Some (self . array . len ())) } }
    };
}

impl_6!();