macro_rules! deps {
    () => {
        BuildSubscriberRet!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < T : tracing :: Subscriber + for < 'span > tracing_subscriber :: registry :: LookupSpan < 'span > + Send + Sync , > BuildSubscriberRet for T { }
    };
}

impl_4!()