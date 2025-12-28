macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T > Stream for Once < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < T > > { Pin :: new (& mut self . iter) . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_58!();