macro_rules! deps {
    () => {
        Subscriber!();
        Kind!();
    };
}

macro_rules! Registrar {
    () => {
        deps!();
        # [cfg (feature = "std")] pub (crate) struct Registrar (Kind < Weak < dyn Subscriber + Send + Sync > >) ;
    };
}

Registrar!();