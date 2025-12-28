macro_rules! deps {
    () => {
        Kind!();
        Subscriber!();
    };
}

macro_rules! Dispatch {
    () => {
        deps!();
        # [doc = " `Dispatch` trace data to a [`Subscriber`]."] # [derive (Clone)] pub struct Dispatch { subscriber : Kind < Arc < dyn Subscriber + Send + Sync > > , }
    };
}

Dispatch!()