macro_rules! deps {
    () => {
        Handle!();
        SelfPipeWrite!();
    };
}

macro_rules! AddSignal {
    () => {
        deps!();
        # [doc = " An internal trait to hide adding new signals into a Handle behind a dynamic dispatch."] trait AddSignal : Debug + Send + Sync { fn add_signal (self : Arc < Self > , write : Arc < dyn SelfPipeWrite > , signal : c_int ,) -> Result < SigId , Error > ; }
    };
}

AddSignal!()