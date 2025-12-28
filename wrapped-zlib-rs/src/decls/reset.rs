macro_rules! deps {
    () => {
        ReturnCode!();
        InflateStream!();
    };
}

macro_rules! reset {
    () => {
        deps!();
        pub fn reset (stream : & mut InflateStream) -> ReturnCode { stream . state . window . clear () ; stream . state . error_message = None ; reset_keep (stream) }
    };
}

reset!();