macro_rules! deps {
    () => {
        Iter!();
        Pending!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < I > Stream for Iter < I > where I : Iterator , { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < I :: Item > > { if self . yield_amt >= 32 { self . yield_amt = 0 ; cx . waker () . wake_by_ref () ; Poll :: Pending } else { self . yield_amt += 1 ; Poll :: Ready (self . iter . next ()) } } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_53!()