macro_rules! deps {
    () => {
        PollSender!();
        PollSenderFuture!();
        State!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > Clone for PollSender < T > { # [doc = " Clones this `PollSender`."] # [doc = ""] # [doc = " The resulting `PollSender` will have an initial state identical to calling `PollSender::new`."] fn clone (& self) -> PollSender < T > { let (sender , state) = match self . sender . clone () { Some (sender) => (Some (sender . clone ()) , State :: Idle (sender)) , None => (None , State :: Closed) , } ; Self { sender , state , acquire : PollSenderFuture :: empty () , } } }
    };
}

impl_67!();