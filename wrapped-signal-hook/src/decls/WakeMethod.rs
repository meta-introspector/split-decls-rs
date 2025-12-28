macro_rules! WakeMethod {
    () => {
        # [derive (Copy , Clone)] pub (crate) enum WakeMethod { Send , Write , }
    };
}

WakeMethod!();