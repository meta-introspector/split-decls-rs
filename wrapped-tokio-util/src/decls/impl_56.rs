macro_rules! deps {
    () => {
        PollSendError!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < T > PollSendError < T > { # [doc = " Consumes the stored value, if any."] # [doc = ""] # [doc = " If this error was encountered when calling `start_send`/`send_item`, this will be the item"] # [doc = " that the caller attempted to send.  Otherwise, it will be `None`."] pub fn into_inner (self) -> Option < T > { self . 0 } }
    };
}

impl_56!();