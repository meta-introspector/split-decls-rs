macro_rules! deps {
    () => {
        InnerFuture!();
    };
}

macro_rules! PollSenderFuture {
    () => {
        deps!();
        # [derive (Debug)] struct PollSenderFuture < T > (InnerFuture < 'static , T >) ;
    };
}

PollSenderFuture!()