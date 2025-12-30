// Generated macro for impl_148 (impl)
macro_rules! Depcrate_codecimpl_148 {
() => {
// Module: crate::codec
// Provides: {"impl_148"}
// Dependencies: {}
impl Codec for Bincode { fn encode < I > (input : I) -> JsValue where I : Serialize , { let buf = bincode :: serialize (& input) . expect ("can't serialize an worker message") ; Uint8Array :: from (buf . as_slice ()) . into () } fn decode < O > (input : JsValue) -> O where O : for < 'de > Deserialize < 'de > , { let data = Uint8Array :: from (input) . to_vec () ; bincode :: deserialize (& data) . expect ("can't deserialize an worker message") } }
};
}
