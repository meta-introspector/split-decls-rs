// Generated macro for InstrumentedService (type)
macro_rules! DepcrateInstrumentedService {
() => {
// Module: crate
// Provides: {"InstrumentedService"}
// Dependencies: {}
pub type InstrumentedService < S , R > = service_span :: Service < request_span :: Service < S , R > > ;
};
}
