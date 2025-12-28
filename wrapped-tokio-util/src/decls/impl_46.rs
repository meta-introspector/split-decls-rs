macro_rules! deps {
    () => {
        CancellationToken!();
        MaybeDangling!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl WaitForCancellationFutureOwned { fn new (cancellation_token : CancellationToken) -> Self { WaitForCancellationFutureOwned { future : MaybeDangling :: new (unsafe { Self :: new_future (& cancellation_token) }) , cancellation_token , } } # [doc = " # Safety"] # [doc = " The returned future must be destroyed before the cancellation token is"] # [doc = " destroyed."] unsafe fn new_future (cancellation_token : & CancellationToken ,) -> tokio :: sync :: futures :: Notified < 'static > { let inner_ptr = Arc :: as_ptr (& cancellation_token . inner) ; (* inner_ptr) . notified () } }
    };
}

impl_46!();