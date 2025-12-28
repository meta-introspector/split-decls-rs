macro_rules! deps {
    () => {
        Elaboratable!();
        Elaborator!();
        Interner!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < I : Interner , O : Elaboratable < I > > Iterator for Elaborator < I , O > { type Item = O ; fn size_hint (& self) -> (usize , Option < usize >) { (self . stack . len () , None) } fn next (& mut self) -> Option < Self :: Item > { if let Some (obligation) = self . stack . pop () { self . elaborate (& obligation) ; Some (obligation) } else { None } } }
    };
}

impl_24!()