macro_rules! deps {
    () => {
        Handle!();
        Message!();
    };
}

macro_rules! Sender {
    () => {
        deps!();
        # [doc = " Type that will receive [`Messages`](Message) from a [`Handle`]."] pub type Sender = crossbeam_channel :: Sender < Message > ;
    };
}

Sender!();