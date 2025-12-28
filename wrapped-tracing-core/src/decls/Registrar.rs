macro_rules! deps {
    () => {
        Kind!();
        Subscriber!();
    };
}

macro_rules! Registrar {
    () => {
        deps!();
        # [cfg (feature = "std")] pub (crate) struct Registrar (Kind < Weak < dyn Subscriber + Send + Sync > >) ;
    };
}

Registrar!()