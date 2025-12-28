macro_rules! deps {
    () => {
        Mock!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl AsyncRead for Mock { fn poll_read (mut self : Pin < & mut Self > , cx : & mut task :: Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { loop { if let Some (ref mut sleep) = self . inner . sleep { ready ! (Pin :: new (sleep) . poll (cx)) ; } self . inner . sleep = None ; let filled = buf . filled () . len () ; match self . inner . read (buf) { Err (ref e) if e . kind () == io :: ErrorKind :: WouldBlock => { if let Some (rem) = self . inner . remaining_wait () { let until = Instant :: now () + rem ; self . inner . sleep = Some (Box :: pin (time :: sleep_until (until))) ; } else { self . inner . read_wait = Some (cx . waker () . clone ()) ; return Poll :: Pending ; } } Ok (()) => { if buf . filled () . len () == filled { match ready ! (self . inner . poll_action (cx)) { Some (action) => { self . inner . actions . push_back (action) ; continue ; } None => { return Poll :: Ready (Ok (())) ; } } } else { return Poll :: Ready (Ok (())) ; } } Err (e) => return Poll :: Ready (Err (e)) , } } } }
    };
}

impl_9!();