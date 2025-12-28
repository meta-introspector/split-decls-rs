macro_rules! deps {
    () => {
        Error!();
        Outcome!();
    };
}

macro_rules! OutcomeTrait {
    () => {
        deps!();
        # [doc = " This trait allows us to have two different Outcome types:"] # [doc = "  - the normal one that does as little as possible"] # [doc = "  - one for tests that does some additional work and checking"] pub trait OutcomeTrait { type Error ; type Obligation ; fn new () -> Self ; fn record_completed (& mut self , outcome : & Self :: Obligation) ; fn record_error (& mut self , error : Self :: Error) ; }
    };
}

OutcomeTrait!();