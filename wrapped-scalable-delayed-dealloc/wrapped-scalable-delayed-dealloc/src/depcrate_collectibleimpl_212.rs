// Generated macro for impl_212 (impl)
macro_rules! Depcrate_collectibleimpl_212 {
() => {
// Module: crate::collectible
// Provides: {"impl_212"}
// Dependencies: {}
impl Collectible for Link { # [inline] fn next_ptr (& self) -> Option < NonNull < dyn Collectible > > { let fat_ptr : (* mut usize , * mut usize) = (self . data . 0 . load (Relaxed) as * mut usize , self . data . 1 . load (Relaxed) ,) ; unsafe { std :: mem :: transmute (fat_ptr) } } # [inline] fn set_next_ptr (& self , next_ptr : Option < NonNull < dyn Collectible > >) { let data : (* mut usize , * mut usize) = next_ptr . map_or_else (| | (ptr :: null_mut () , ptr :: null_mut ()) , | p | unsafe { std :: mem :: transmute (p) } ,) ; self . data . 0 . store (data . 0 as usize , Relaxed) ; self . data . 1 . store (data . 1 , Relaxed) ; } }
};
}
