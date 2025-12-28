macro_rules! deps {
    () => {
        Spawn!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < T : Stream > Stream for Spawn < T > { type Item = T :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . future . as_mut () . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . future . size_hint () } }
    };
}

impl_48!()