macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T > Stream for Empty < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Ready (None) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_48!();