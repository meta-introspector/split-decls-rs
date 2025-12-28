macro_rules! deps {
    () => {
        Subscriber!();
        Kind!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl Kind < Weak < dyn Subscriber + Send + Sync > > { fn upgrade (& self) -> Option < Kind < Arc < dyn Subscriber + Send + Sync > > > { match self { Kind :: Global (s) => Some (Kind :: Global (* s)) , Kind :: Scoped (ref s) => Some (Kind :: Scoped (s . upgrade () ?)) , } } }
    };
}

impl_94!()