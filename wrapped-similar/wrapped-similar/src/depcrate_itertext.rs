// Generated macro for text (module)
macro_rules! Depcrate_itertext {
() => {
// Module: crate::iter
// Provides: {"text"}
// Dependencies: {}
# [cfg (feature = "text")] mod text { use super :: * ; # [doc = " Iterator for [`TextDiff::iter_all_changes`](crate::TextDiff::iter_all_changes)."] pub struct AllChangesIter < 'slf , 'data , T : ? Sized > { old : & 'slf [& 'data T] , new : & 'slf [& 'data T] , ops : & 'slf [DiffOp] , current_iter : Option < ChangesIter < 'slf , [& 'data T] , [& 'data T] , & 'data T > > , } impl < 'slf , 'data , T > AllChangesIter < 'slf , 'data , T > where T : 'data + ? Sized + PartialEq , { pub (crate) fn new (old : & 'slf [& 'data T] , new : & 'slf [& 'data T] , ops : & 'slf [DiffOp] ,) -> Self { AllChangesIter { old , new , ops , current_iter : None , } } } impl < 'slf , 'data , T > Iterator for AllChangesIter < 'slf , 'data , T > where T : PartialEq + 'data + ? Sized , 'data : 'slf , { type Item = Change < & 'data T > ; fn next (& mut self) -> Option < Self :: Item > { loop { if let Some (ref mut iter) = self . current_iter { if let Some (rv) = iter . next () { return Some (rv) ; } self . current_iter . take () ; } if let Some ((& first , rest)) = self . ops . split_first () { self . current_iter = Some (ChangesIter :: new (self . old , self . new , first)) ; self . ops = rest ; } else { return None ; } } } } }
};
}
