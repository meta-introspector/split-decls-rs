macro_rules! deps {
    () => {
        Message!();
    };
}

macro_rules! NotifyHandle {
    () => {
        deps!();
        # [derive (Debug)] pub struct NotifyHandle { sender : Sender < Message > , _thread : stdx :: thread :: JoinHandle , }
    };
}

NotifyHandle!()