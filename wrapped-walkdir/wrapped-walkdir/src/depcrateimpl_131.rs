// Generated macro for impl_131 (impl)
macro_rules! Depcrateimpl_131 {
() => {
// Module: crate
// Provides: {"impl_131"}
// Dependencies: {}
impl Iterator for DirList { type Item = Result < DirEntry > ; # [inline (always)] fn next (& mut self) -> Option < Result < DirEntry > > { match * self { DirList :: Closed (ref mut it) => it . next () , DirList :: Opened { depth , ref mut it } => match * it { Err (ref mut err) => err . take () . map (Err) , Ok (ref mut rd) => rd . next () . map (| r | match r { Ok (r) => DirEntry :: from_entry (depth + 1 , & r) , Err (err) => Err (Error :: from_io (depth + 1 , err)) , }) , } , } } }
};
}
