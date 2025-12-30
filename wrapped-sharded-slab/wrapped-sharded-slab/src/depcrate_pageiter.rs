// Generated macro for Iter (type)
macro_rules! Depcrate_pageIter {
() => {
// Module: crate::page
// Provides: {"Iter"}
// Dependencies: {}
pub (crate) type Iter < 'a , T , C > = std :: iter :: FilterMap < std :: slice :: Iter < 'a , Slot < Option < T > , C > > , fn (& 'a Slot < Option < T > , C >) -> Option < & 'a T > , > ;
};
}
