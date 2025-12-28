macro_rules! impl_45 {
    () => {
        impl < T : Future > Future for Instrumented < T > { type Output = T :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; inner . poll (cx) } }
    };
}

impl_45!()