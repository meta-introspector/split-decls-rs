macro_rules! deps {
    () => {
        PollSemaphore!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Stream for PollSemaphore { type Item = OwnedSemaphorePermit ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < OwnedSemaphorePermit > > { Pin :: into_inner (self) . poll_acquire (cx) } }
    };
}

impl_72!();