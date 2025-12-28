macro_rules! deps {
    () => {
        EventReceiver!();
        Source!();
    };
}

macro_rules! ValidateWhitespace {
    () => {
        deps!();
        # [doc = " Centralize validation for all whitespace-like content"] pub struct ValidateWhitespace < 'r , 's > { receiver : & 'r mut dyn EventReceiver , source : Source < 's > , }
    };
}

ValidateWhitespace!();