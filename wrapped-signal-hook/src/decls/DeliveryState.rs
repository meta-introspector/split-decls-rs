macro_rules! DeliveryState {
    () => {
        # [derive (Debug)] struct DeliveryState { closed : AtomicBool , registered_signal_ids : Mutex < Vec < Option < SigId > > > , }
    };
}

DeliveryState!()