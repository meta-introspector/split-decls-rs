macro_rules! deps {
    () => {
        CountLatchKind!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl std :: fmt :: Debug for CountLatchKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { CountLatchKind :: Stealing { latch , .. } => { f . debug_tuple ("Stealing") . field (latch) . finish () } CountLatchKind :: Blocking { latch , .. } => { f . debug_tuple ("Blocking") . field (latch) . finish () } } } }
    };
}

impl_89!()