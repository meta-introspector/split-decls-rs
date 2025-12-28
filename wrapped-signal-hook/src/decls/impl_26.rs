macro_rules! deps {
    () => {
        SignalIterator!();
        SignalDelivery!();
        Pending!();
        PollResult!();
        Exfiltrator!();
        Handle!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < SD , E : Exfiltrator > SignalIterator < SD , E > { # [doc = " Create a new infinite iterator for signals registered with the passed"] # [doc = " in [`SignalDelivery`] instance."] pub fn new < R > (mut signals : SD) -> Self where SD : BorrowMut < SignalDelivery < R , E > > , R : 'static + AsRawFd + Send + Sync , { let iter = signals . borrow_mut () . pending () ; Self { signals , iter } } # [doc = " Return a signal if there is one or tell the caller that there is none at the moment."] # [doc = ""] # [doc = " You have to pass in a callback which checks the underlying reading end of the pipe if"] # [doc = " there may be any pending signals. This callback may or may not block. If the callback"] # [doc = " returns [`true`] this method will try to fetch the next signal and return it as a"] # [doc = " [`PollResult::Signal`]. If the callback returns [`false`] the method will return"] # [doc = " [`PollResult::Pending`] and assume it will be called again at a later point in time."] # [doc = " The callback may be called any number of times by this function."] # [doc = ""] # [doc = " If the iterator was closed by the [`close`][Handle::close] method of the associated"] # [doc = " [`Handle`] this method will return [`PollResult::Closed`]."] pub fn poll_signal < R , F > (& mut self , has_signals : & mut F) -> PollResult < E :: Output > where SD : BorrowMut < SignalDelivery < R , E > > , R : 'static + AsRawFd + Send + Sync , F : FnMut (& mut R) -> Result < bool , Error > , { while ! self . signals . borrow_mut () . handle . is_closed () { if let Some (result) = self . iter . next () { return PollResult :: Signal (result) ; } match self . signals . borrow_mut () . poll_pending (has_signals) { Ok (Some (pending)) => self . iter = pending , Ok (None) => return PollResult :: Pending , Err (err) => return PollResult :: Err (err) , } } PollResult :: Closed } # [doc = " Get a shareable [`Handle`] for this instance."] # [doc = ""] # [doc = " This can be used to add further signals or terminate the whole"] # [doc = " signal iteration using the [`close`][Handle::close] method."] pub fn handle < R > (& self) -> Handle where SD : Borrow < SignalDelivery < R , E > > , R : 'static + AsRawFd + Send + Sync , { self . signals . borrow () . handle () } }
    };
}

impl_26!()