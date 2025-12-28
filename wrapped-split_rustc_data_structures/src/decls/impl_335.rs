macro_rules! deps {
    () => {
        Error!();
        OutcomeTrait!();
        Outcome!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        impl < O , E > OutcomeTrait for Outcome < O , E > { type Error = Error < O , E > ; type Obligation = O ; fn new () -> Self { Self { errors : vec ! [] } } fn record_completed (& mut self , _outcome : & Self :: Obligation) { } fn record_error (& mut self , error : Self :: Error) { self . errors . push (error) } }
    };
}

impl_335!();