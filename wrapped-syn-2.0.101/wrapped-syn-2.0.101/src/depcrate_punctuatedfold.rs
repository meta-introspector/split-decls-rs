// Generated macro for fold (function)
macro_rules! Depcrate_punctuatedfold {
() => {
// Module: crate::punctuated
// Provides: {"fold"}
// Dependencies: {}
# [cfg (all (feature = "fold" , any (feature = "full" , feature = "derive")))] pub (crate) fn fold < T , P , V , F > (punctuated : Punctuated < T , P > , fold : & mut V , mut f : F ,) -> Punctuated < T , P > where V : ? Sized , F : FnMut (& mut V , T) -> T , { Punctuated { inner : punctuated . inner . into_iter () . map (| (t , p) | (f (fold , t) , p)) . collect () , last : match punctuated . last { Some (t) => Some (Box :: new (f (fold , * t))) , None => None , } , } }
};
}
