// Generated macro for pid_decl (macro)
macro_rules! Depcrate_common_systempid_decl {
() => {
// Module: crate::common::system
// Provides: {"pid_decl"}
// Dependencies: {}
macro_rules ! pid_decl { ($ typ : ty) => { # [doc = include_str ! ("../../md_doc/pid.md")] # [derive (Clone , Copy , Debug , Hash , PartialEq , Eq , PartialOrd , Ord)] # [repr (transparent)] pub struct Pid (pub (crate) $ typ) ; impl From < usize > for Pid { fn from (v : usize) -> Self { Self (v as _) } } impl From < Pid > for usize { fn from (v : Pid) -> Self { v . 0 as _ } } impl FromStr for Pid { type Err = <$ typ as FromStr >:: Err ; fn from_str (s : & str) -> Result < Self , Self :: Err > { Ok (Self (<$ typ >:: from_str (s) ?)) } } impl fmt :: Display for Pid { fn fmt (& self , f : & mut fmt :: Formatter <'_ >) -> fmt :: Result { write ! (f , "{}" , self . 0) } } impl Pid { # [doc = " Allows to convert [`Pid`][crate::Pid] into [`u32`]."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::Pid;"] # [doc = ""] # [doc = " let pid = Pid::from_u32(0);"] # [doc = " let value: u32 = pid.as_u32();"] # [doc = " ```"] pub fn as_u32 (self) -> u32 { self . 0 as _ } # [doc = " Allows to convert a [`u32`] into [`Pid`][crate::Pid]."] # [doc = ""] # [doc = " ```"] # [doc = " use sysinfo::Pid;"] # [doc = ""] # [doc = " let pid = Pid::from_u32(0);"] # [doc = " ```"] pub fn from_u32 (v : u32) -> Self { Self (v as _) } } } ; }
};
}
