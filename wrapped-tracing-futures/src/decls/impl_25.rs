macro_rules! deps {
    () => {
        Instrumented!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [cfg (all (feature = "futures-03" , feature = "std-future"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "futures-03" , feature = "std-future"))))] impl < T : futures :: Stream > futures :: Stream for Instrumented < T > { type Item = T :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> futures :: task :: Poll < Option < Self :: Item > > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; T :: poll_next (inner , cx) } }
    };
}

impl_25!();