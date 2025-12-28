macro_rules! deps {
    () => {
        Subscriber!();
        Kind!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Kind < Arc < dyn Subscriber + Send + Sync > > { fn downgrade (& self) -> Kind < Weak < dyn Subscriber + Send + Sync > > { match self { Kind :: Global (s) => Kind :: Global (* s) , Kind :: Scoped (ref s) => Kind :: Scoped (Arc :: downgrade (s)) , } } }
    };
}

impl_93!()