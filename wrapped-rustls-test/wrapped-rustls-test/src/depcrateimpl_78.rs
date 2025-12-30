// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < 'a , C , S > OtherSession < 'a , C , S > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , S : SideData , { pub fn new (sess : & 'a mut C) -> Self { OtherSession { sess , reads : 0 , writevs : vec ! [] , fail_ok : false , short_writes : false , last_error : None , buffered : false , buffer : vec ! [] , } } pub fn new_buffered (sess : & 'a mut C) -> Self { let mut os = OtherSession :: new (sess) ; os . buffered = true ; os } pub fn new_fails (sess : & 'a mut C) -> Self { let mut os = OtherSession :: new (sess) ; os . fail_ok = true ; os } fn flush_vectored (& mut self , b : & [io :: IoSlice < '_ >]) -> io :: Result < usize > { let mut total = 0 ; let mut lengths = vec ! [] ; for bytes in b { let write_len = if self . short_writes { if bytes . len () > 5 { bytes . len () / 2 } else { bytes . len () } } else { bytes . len () } ; let l = self . sess . read_tls (& mut io :: Cursor :: new (& bytes [.. write_len])) ? ; lengths . push (l) ; total += l ; if bytes . len () != l { break ; } } let rc = self . sess . process_new_packets () ; if ! self . fail_ok { rc . unwrap () ; } else if rc . is_err () { self . last_error = rc . err () ; } self . writevs . push (lengths) ; Ok (total) } }
};
}
