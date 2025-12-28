macro_rules! deps {
    () => {
        Subscriber!();
        Kind!();
    };
}

macro_rules! Dispatch {
    () => {
        deps!();
        # [doc = " `Dispatch` trace data to a [`Subscriber`]."] # [derive (Clone)] pub struct Dispatch { subscriber : Kind < Arc < dyn Subscriber + Send + Sync > > , }
    };
}

Dispatch!();