// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl < C , S > io :: Read for OtherSession < '_ , C , S > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , S : SideData , { fn read (& mut self , mut b : & mut [u8]) -> io :: Result < usize > { self . reads += 1 ; self . sess . write_tls (& mut b) } }
};
}
