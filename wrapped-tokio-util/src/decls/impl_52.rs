macro_rules! impl_52 {
    () => {
        impl < F : Future > Future for RunUntilCancelledFutureOwned < F > { type Output = Option < F :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (res) = this . future . poll (cx) { Poll :: Ready (Some (res)) } else if this . cancellation . poll (cx) . is_ready () { Poll :: Ready (None) } else { Poll :: Pending } } }
    };
}

impl_52!()