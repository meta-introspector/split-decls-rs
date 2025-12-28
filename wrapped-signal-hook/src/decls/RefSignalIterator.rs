macro_rules! deps {
    () => {
        SignalDelivery!();
        SignalIterator!();
    };
}

macro_rules! RefSignalIterator {
    () => {
        deps!();
        # [doc = " A signal iterator which takes a mutable reference to a [`SignalDelivery`]"] # [doc = " instance."] pub type RefSignalIterator < 'a , R , E > = SignalIterator < & 'a mut SignalDelivery < R , E > , E > ;
    };
}

RefSignalIterator!();