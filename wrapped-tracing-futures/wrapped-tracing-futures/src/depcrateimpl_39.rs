// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
# [cfg (all (feature = "futures-03" , feature = "std-future"))] # [cfg_attr (docsrs , doc (cfg (all (feature = "futures-03" , feature = "std-future"))))] impl < I , T : futures :: Sink < I > > futures :: Sink < I > for Instrumented < T > where T : futures :: Sink < I > , { type Error = T :: Error ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> futures :: task :: Poll < Result < () , Self :: Error > > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; T :: poll_ready (inner , cx) } fn start_send (self : Pin < & mut Self > , item : I) -> Result < () , Self :: Error > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; T :: start_send (inner , item) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> futures :: task :: Poll < Result < () , Self :: Error > > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; T :: poll_flush (inner , cx) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> futures :: task :: Poll < Result < () , Self :: Error > > { let (span , inner) = self . project () . span_and_inner_pin_mut () ; let _enter = span . enter () ; T :: poll_close (inner , cx) } }
};
}
