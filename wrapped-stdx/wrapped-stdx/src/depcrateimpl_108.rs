// Generated macro for impl_108 (impl)
macro_rules! Depcrateimpl_108 {
() => {
// Module: crate
// Provides: {"impl_108"}
// Dependencies: {}
impl JodChild { pub fn spawn (mut command : Command) -> sio :: Result < Self > { command . spawn () . map (Self) } # [must_use] # [cfg (not (target_arch = "wasm32"))] pub fn into_inner (self) -> std :: process :: Child { unsafe { std :: mem :: transmute :: < Self , std :: process :: Child > (self) } } }
};
}
