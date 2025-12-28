macro_rules! deps {
    () => {
        Mock!();
        Action!();
    };
}

macro_rules! Handle {
    () => {
        deps!();
        # [doc = " A handle to send additional actions to the related `Mock`."] # [derive (Debug)] pub struct Handle { tx : mpsc :: UnboundedSender < Action > , }
    };
}

Handle!()