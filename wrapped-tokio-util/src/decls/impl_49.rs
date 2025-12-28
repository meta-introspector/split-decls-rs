macro_rules! deps {
    () => {
        CancellationToken!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl < 'a , F : Future > RunUntilCancelledFuture < 'a , F > { pub (crate) fn new (cancellation_token : & 'a CancellationToken , future : F) -> Self { Self { cancellation : cancellation_token . cancelled () , future , } } }
    };
}

impl_49!()