macro_rules! deps {
    () => {
        PollSender!();
    };
}

macro_rules! PollSendError {
    () => {
        deps!();
        # [doc = " Error returned by the `PollSender` when the channel is closed."] # [derive (Debug)] pub struct PollSendError < T > (Option < T >) ;
    };
}

PollSendError!()