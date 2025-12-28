macro_rules! deps {
    () => {
        PendingSignals!();
        AddSignal!();
        Exfiltrator!();
        SelfPipeWrite!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < E : Exfiltrator > AddSignal for PendingSignals < E > { fn add_signal (self : Arc < Self > , write : Arc < dyn SelfPipeWrite > , signal : c_int ,) -> Result < SigId , Error > { assert ! (signal >= 0) ; assert ! ((signal as usize) < MAX_SIGNUM , "Signal number {} too large. If your OS really supports such signal, file a bug" , signal ,) ; assert ! (self . exfiltrator . supports_signal (signal) , "Signal {} not supported by exfiltrator {:?}" , signal , self . exfiltrator ,) ; self . exfiltrator . init (& self . slots [signal as usize] , signal) ; let action = move | act : & _ | { let slot = & self . slots [signal as usize] ; let ex = & self . exfiltrator ; ex . store (slot , signal , act) ; write . wake_readers () ; } ; let id = unsafe { signal_hook_registry :: register_sigaction (signal , action) } ? ; Ok (id) } }
    };
}

impl_16!();