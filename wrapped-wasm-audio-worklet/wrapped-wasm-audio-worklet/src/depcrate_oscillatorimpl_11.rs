// Generated macro for impl_11 (impl)
macro_rules! Depcrate_oscillatorimpl_11 {
() => {
// Module: crate::oscillator
// Provides: {"impl_11"}
// Dependencies: {}
impl Oscillator { pub fn process (& mut self , output : & mut [f32]) -> bool { for a in output { let frequency = self . params . frequency . load (Ordering :: Relaxed) ; let volume = self . params . volume . load (Ordering :: Relaxed) ; self . accumulator += u32 :: from (frequency) ; * a = (self . accumulator as f32 / 512.) . sin () * (volume as f32 / 100.) ; } true } }
};
}
