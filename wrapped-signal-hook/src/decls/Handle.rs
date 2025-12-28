macro_rules! deps {
    () => {
        AddSignal!();
        DeliveryState!();
        SelfPipeWrite!();
    };
}

macro_rules! Handle {
    () => {
        deps!();
        # [doc = " A struct to control an instance of an associated type"] # [doc = " (like for example [`Signals`][super::Signals])."] # [doc = ""] # [doc = " It allows to register more signal handlers and to shutdown the signal"] # [doc = " delivery. You can [`clone`][Handle::clone] this type which isn't a"] # [doc = " very expensive operation. The cloned instances can be shared between"] # [doc = " multiple threads."] # [derive (Debug , Clone)] pub struct Handle { pending : Arc < dyn AddSignal > , write : Arc < dyn SelfPipeWrite > , delivery_state : Arc < DeliveryState > , }
    };
}

Handle!()