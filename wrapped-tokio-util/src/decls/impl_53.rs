macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < F : Future > RunUntilCancelledFutureOwned < F > { pub (crate) fn new (cancellation_token : CancellationToken , future : F) -> Self { Self { cancellation : cancellation_token . cancelled_owned () , future , } } }
    };
}

impl_53!();