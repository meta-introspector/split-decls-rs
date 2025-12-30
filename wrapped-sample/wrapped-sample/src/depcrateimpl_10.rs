// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl Timeout { pub fn new (dur : Duration) -> Timeout { let millis = dur . as_secs () . checked_mul (1000) . unwrap () . checked_add (dur . subsec_millis () as u64) . unwrap () as f64 ; let mut id = None ; let promise = Promise :: new (& mut | resolve , _reject | { id = Some (set_timeout (resolve . into () , millis)) ; }) ; Timeout { id : id . unwrap () , inner : JsFuture :: from (promise) , } } }
};
}
