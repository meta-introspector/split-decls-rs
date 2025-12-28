macro_rules! deps {
    () => {
        PollSenderFuture!();
        ReusableBoxFuture!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T > PollSenderFuture < T > { # [doc = " Create with an empty inner future with no `Send` bound."] fn empty () -> Self { Self (ReusableBoxFuture :: new (async { unreachable ! () })) } }
    };
}

impl_64!()