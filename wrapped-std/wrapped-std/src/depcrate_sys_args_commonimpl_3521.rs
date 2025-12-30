// Generated macro for impl_3521 (impl)
macro_rules! Depcrate_sys_args_commonimpl_3521 {
() => {
// Module: crate::sys::args::common
// Provides: {"impl_3521"}
// Dependencies: {}
impl Iterator for Args { type Item = OsString ; # [inline] fn next (& mut self) -> Option < OsString > { self . iter . next () } # [inline] fn next_chunk < const N : usize > (& mut self ,) -> Result < [OsString ; N] , array :: IntoIter < OsString , N > > { self . iter . next_chunk () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } # [inline] fn count (self) -> usize { self . iter . len () } # [inline] fn last (self) -> Option < OsString > { self . iter . last () } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . iter . advance_by (n) } # [inline] fn try_fold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . iter . try_fold (init , f) } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . iter . fold (init , f) } }
};
}
