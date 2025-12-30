// Generated macro for impl_20 (impl)
macro_rules! Depcrate_wasm_audioimpl_20 {
() => {
// Module: crate::wasm_audio
// Provides: {"impl_20"}
// Dependencies: {}
# [wasm_bindgen] impl WasmAudioProcessor { pub fn process (& mut self , buf : & mut [f32]) -> bool { self . 0 (buf) } pub fn pack (self) -> usize { Box :: into_raw (Box :: new (self)) as usize } pub unsafe fn unpack (val : usize) -> Self { * Box :: from_raw (val as * mut _) } }
};
}
