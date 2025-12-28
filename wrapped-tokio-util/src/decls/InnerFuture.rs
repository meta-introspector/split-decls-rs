macro_rules! deps {
    () => {
        ReusableBoxFuture!();
        PollSendError!();
    };
}

macro_rules! InnerFuture {
    () => {
        deps!();
        type InnerFuture < 'a , T > = ReusableBoxFuture < 'a , Result < OwnedPermit < T > , PollSendError < T > > > ;
    };
}

InnerFuture!()