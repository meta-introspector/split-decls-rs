macro_rules! deps {
    () => {
        Mock!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl AsyncWrite for Mock { fn poll_write (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ > , buf : & [u8] ,) -> Poll < io :: Result < usize > > { loop { if let Some (ref mut sleep) = self . inner . sleep { ready ! (Pin :: new (sleep) . poll (cx)) ; } self . inner . sleep = None ; if self . inner . actions . is_empty () { match self . inner . poll_action (cx) { Poll :: Pending => { } Poll :: Ready (Some (action)) => { self . inner . actions . push_back (action) ; } Poll :: Ready (None) => { panic ! ("unexpected write {}" , self . pmsg ()) ; } } } match self . inner . write (buf) { Err (ref e) if e . kind () == io :: ErrorKind :: WouldBlock => { if let Some (rem) = self . inner . remaining_wait () { let until = Instant :: now () + rem ; self . inner . sleep = Some (Box :: pin (time :: sleep_until (until))) ; } else { panic ! ("unexpected WouldBlock {}" , self . pmsg ()) ; } } Ok (0) => { if ! self . inner . actions . is_empty () { return Poll :: Pending ; } match ready ! (self . inner . poll_action (cx)) { Some (action) => { self . inner . actions . push_back (action) ; continue ; } None => { panic ! ("unexpected write {}" , self . pmsg ()) ; } } } ret => { self . maybe_wakeup_reader () ; return Poll :: Ready (ret) ; } } } } fn poll_flush (self : Pin < & mut Self > , _cx : & mut task :: Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } fn poll_shutdown (self : Pin < & mut Self > , _cx : & mut task :: Context < '_ >) -> Poll < io :: Result < () > > { Poll :: Ready (Ok (())) } }
    };
}

impl_10!();