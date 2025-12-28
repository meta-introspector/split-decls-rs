macro_rules! deps {
    () => {
        SignalsInfo!();
        Signals!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < E : Exfiltrator > SignalsInfo < E > { # [doc = " Create a `Signals` instance."] # [doc = ""] # [doc = " This registers all the signals listed. The same restrictions (panics, errors) apply"] # [doc = " as with [`Handle::add_signal`]."] pub fn new < I , S > (signals : I) -> Result < Self , Error > where I : IntoIterator < Item = S > , S : Borrow < c_int > , E : Default , { Self :: with_exfiltrator (signals , E :: default ()) } # [doc = " A constructor with explicit exfiltrator."] pub fn with_exfiltrator < I , S > (signals : I , exfiltrator : E) -> Result < Self , Error > where I : IntoIterator < Item = S > , S : Borrow < c_int > , { let (read , write) = Async :: < UnixStream > :: pair () ? ; let inner = SignalDelivery :: with_pipe (read , write , exfiltrator , signals) ? ; Ok (Self (OwningSignalIterator :: new (inner))) } # [doc = " Get a shareable [`Handle`] for this `Signals` instance."] # [doc = ""] # [doc = " This can be used to add further signals or close the [`Signals`] instance"] # [doc = " which terminates the whole signal stream."] pub fn handle (& self) -> Handle { self . 0 . handle () } }
    };
}

impl_1!()