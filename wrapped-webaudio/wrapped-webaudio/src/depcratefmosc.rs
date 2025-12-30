// Generated macro for FmOsc (struct)
macro_rules! DepcrateFmOsc {
() => {
// Module: crate
// Provides: {"FmOsc"}
// Dependencies: {}
# [wasm_bindgen] pub struct FmOsc { ctx : AudioContext , # [doc = " The primary oscillator.  This will be the fundamental frequency"] primary : web_sys :: OscillatorNode , # [doc = " Overall gain (volume) control"] gain : web_sys :: GainNode , # [doc = " Amount of frequency modulation"] fm_gain : web_sys :: GainNode , # [doc = " The oscillator that will modulate the primary oscillator's frequency"] fm_osc : web_sys :: OscillatorNode , # [doc = " The ratio between the primary frequency and the fm_osc frequency."] # [doc = ""] # [doc = " Generally fractional values like 1/2 or 1/4 sound best"] fm_freq_ratio : f32 , fm_gain_ratio : f32 , }
};
}
