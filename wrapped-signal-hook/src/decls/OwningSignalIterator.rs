macro_rules! deps {
    () => {
        SignalDelivery!();
        SignalIterator!();
    };
}

macro_rules! OwningSignalIterator {
    () => {
        deps!();
        # [doc = " A signal iterator which consumes a [`SignalDelivery`] instance and takes"] # [doc = " ownership of it."] pub type OwningSignalIterator < R , E > = SignalIterator < SignalDelivery < R , E > , E > ;
    };
}

OwningSignalIterator!();