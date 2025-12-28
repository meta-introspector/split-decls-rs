macro_rules! deps {
    () => {
        MaybeDangling!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Future for WaitForCancellationFutureOwned { type Output = () ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < () > { let mut this = self . project () ; loop { if this . cancellation_token . is_cancelled () { return Poll :: Ready (()) ; } if this . future . as_mut () . poll (cx) . is_pending () { return Poll :: Pending ; } this . future . set (MaybeDangling :: new (unsafe { Self :: new_future (this . cancellation_token) })) ; } } }
    };
}

impl_47!()