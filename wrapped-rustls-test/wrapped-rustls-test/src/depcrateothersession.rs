// Generated macro for OtherSession (struct)
macro_rules! DepcrateOtherSession {
() => {
// Module: crate
// Provides: {"OtherSession"}
// Dependencies: {}
pub struct OtherSession < 'a , C , S > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , S : SideData , { sess : & 'a mut C , pub reads : usize , pub writevs : Vec < Vec < usize > > , fail_ok : bool , pub short_writes : bool , pub last_error : Option < Error > , pub buffered : bool , buffer : Vec < Vec < u8 > > , }
};
}
