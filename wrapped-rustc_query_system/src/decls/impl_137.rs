macro_rules! deps {
    () => {
        QueryResult!();
        QueryJob!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < I > QueryResult < I > { # [doc = " Unwraps the query job expecting that it has started."] fn expect_job (self) -> QueryJob < I > { match self { Self :: Started (job) => job , Self :: Poisoned => { panic ! ("job for query failed to start and was poisoned") } } } }
    };
}

impl_137!();