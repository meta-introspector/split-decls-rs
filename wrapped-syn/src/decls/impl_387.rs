macro_rules! impl_387 {
    () => {
        impl Signature { # [doc = " A method's `self` receiver, such as `&self` or `self: Box<Self>`."] pub fn receiver (& self) -> Option < & Receiver > { let arg = self . inputs . first () ? ; match arg { FnArg :: Receiver (receiver) => Some (receiver) , FnArg :: Typed (_) => None , } } }
    };
}

impl_387!();