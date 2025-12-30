// Generated macro for impl_80 (impl)
macro_rules! Depcrateimpl_80 {
() => {
// Module: crate
// Provides: {"impl_80"}
// Dependencies: {}
impl < C , S > io :: Write for OtherSession < '_ , C , S > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , S : SideData , { fn write (& mut self , _ : & [u8]) -> io :: Result < usize > { unreachable ! () } fn flush (& mut self) -> io :: Result < () > { if ! self . buffer . is_empty () { let buffer = mem :: take (& mut self . buffer) ; let slices = buffer . iter () . map (| b | io :: IoSlice :: new (b)) . collect :: < Vec < _ > > () ; self . flush_vectored (& slices) ? ; } Ok (()) } fn write_vectored (& mut self , b : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { if self . buffered { self . buffer . extend (b . iter () . map (| s | s . to_vec ())) ; return Ok (b . iter () . map (| s | s . len ()) . sum ()) ; } self . flush_vectored (b) } }
};
}
