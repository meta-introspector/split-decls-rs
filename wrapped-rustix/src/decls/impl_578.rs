macro_rules! deps {
    () => {
        RecvAncillaryBuffer!();
    };
}

macro_rules! impl_578 {
    () => {
        deps!();
        impl Drop for RecvAncillaryBuffer < '_ > { fn drop (& mut self) { self . clear () ; } }
    };
}

impl_578!();