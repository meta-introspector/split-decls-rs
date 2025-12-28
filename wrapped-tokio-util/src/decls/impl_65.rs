macro_rules! deps {
    () => {
        PollSendError!();
        InnerFuture!();
        PollSenderFuture!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T : Send > PollSenderFuture < T > { # [doc = " Create with an empty inner future."] fn new () -> Self { let v = InnerFuture :: new (make_acquire_future (None)) ; Self (unsafe { mem :: transmute :: < InnerFuture < '_ , T > , InnerFuture < 'static , T > > (v) }) } # [doc = " Poll the inner future."] fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < Result < OwnedPermit < T > , PollSendError < T > > > { self . 0 . poll (cx) } # [doc = " Replace the inner future."] fn set (& mut self , sender : Option < Sender < T > >) { let inner : * mut InnerFuture < 'static , T > = & mut self . 0 ; let inner : * mut InnerFuture < '_ , T > = inner . cast () ; let inner = unsafe { & mut * inner } ; inner . set (make_acquire_future (sender)) ; } }
    };
}

impl_65!();