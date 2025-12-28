macro_rules! deps {
    () => {
        Exfiltrator!();
        Pending!();
        PendingSignals!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < E : Exfiltrator > Pending < E > { fn new (pending : Arc < PendingSignals < E > >) -> Self { Self { pending , position : 0 , } } }
    };
}

impl_22!()