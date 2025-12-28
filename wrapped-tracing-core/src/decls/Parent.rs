macro_rules! deps {
    () => {
        Current!();
        Id!();
    };
}

macro_rules! Parent {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) enum Parent { # [doc = " The new span will be a root span."] Root , # [doc = " The new span will be rooted in the current span."] Current , # [doc = " The new span has an explicitly-specified parent."] Explicit (Id) , }
    };
}

Parent!()