macro_rules! deps {
    () => {
        PollSenderFuture!();
        State!();
    };
}

macro_rules! PollSender {
    () => {
        deps!();
        # [doc = " A wrapper around [`mpsc::Sender`] that can be polled."] # [doc = ""] # [doc = " [`mpsc::Sender`]: tokio::sync::mpsc::Sender"] # [derive (Debug)] pub struct PollSender < T > { sender : Option < Sender < T > > , state : State < T > , acquire : PollSenderFuture < T > , }
    };
}

PollSender!()