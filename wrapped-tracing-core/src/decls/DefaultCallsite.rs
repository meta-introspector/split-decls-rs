macro_rules! deps {
    () => {
        Callsite!();
        Metadata!();
    };
}

macro_rules! DefaultCallsite {
    () => {
        deps!();
        # [doc = " A default [`Callsite`] implementation."] # [derive (Debug)] pub struct DefaultCallsite { interest : AtomicU8 , registration : AtomicU8 , meta : & 'static Metadata < 'static > , next : AtomicPtr < Self > , }
    };
}

DefaultCallsite!();