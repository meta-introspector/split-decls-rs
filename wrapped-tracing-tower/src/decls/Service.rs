macro_rules! Service {
    () => {
        # [derive (Debug)] pub struct Service < S > { inner : S , span : tracing :: Span , }
    };
}

Service!();