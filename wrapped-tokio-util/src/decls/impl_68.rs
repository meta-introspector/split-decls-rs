macro_rules! deps {
    () => {
        PollSendError!();
        PollSender!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < T : Send > Sink < T > for PollSender < T > { type Error = PollSendError < T > ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: into_inner (self) . poll_reserve (cx) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : T) -> Result < () , Self :: Error > { Pin :: into_inner (self) . send_item (item) } fn poll_close (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: into_inner (self) . close () ; Poll :: Ready (Ok (())) } }
    };
}

impl_68!();