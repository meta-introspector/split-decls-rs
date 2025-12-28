macro_rules! deps {
    () => {
        AsLog!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'a > AsLog for Metadata < 'a > { type Log = log :: Metadata < 'a > ; fn as_log (& self) -> Self :: Log { log :: Metadata :: builder () . level (self . level () . as_log ()) . target (self . target ()) . build () } }
    };
}

impl_7!()