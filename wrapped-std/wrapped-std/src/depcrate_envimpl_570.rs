// Generated macro for impl_570 (impl)
macro_rules! Depcrate_envimpl_570 {
() => {
// Module: crate::env
// Provides: {"impl_570"}
// Dependencies: {}
# [stable (feature = "env" , since = "1.0.0")] impl Iterator for ArgsOs { type Item = OsString ; # [inline] fn next (& mut self) -> Option < OsString > { self . inner . next () } # [inline] fn next_chunk < const N : usize > (& mut self ,) -> Result < [OsString ; N] , array :: IntoIter < OsString , N > > { self . inner . next_chunk () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } # [inline] fn count (self) -> usize { self . inner . len () } # [inline] fn last (self) -> Option < OsString > { self . inner . last () } # [inline] fn advance_by (& mut self , n : usize) -> Result < () , NonZero < usize > > { self . inner . advance_by (n) } # [inline] fn try_fold < B , F , R > (& mut self , init : B , f : F) -> R where F : FnMut (B , Self :: Item) -> R , R : Try < Output = B > , { self . inner . try_fold (init , f) } # [inline] fn fold < B , F > (self , init : B , f : F) -> B where F : FnMut (B , Self :: Item) -> B , { self . inner . fold (init , f) } }
};
}
