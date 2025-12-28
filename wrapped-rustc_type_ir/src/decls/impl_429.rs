macro_rules! deps {
    () => {
        AliasTyKind!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl AliasTyKind { pub fn descr (self) -> & 'static str { match self { AliasTyKind :: Projection => "associated type" , AliasTyKind :: Inherent => "inherent associated type" , AliasTyKind :: Opaque => "opaque type" , AliasTyKind :: Free => "type alias" , } } }
    };
}

impl_429!()