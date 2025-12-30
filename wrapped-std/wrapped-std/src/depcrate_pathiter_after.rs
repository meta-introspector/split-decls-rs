// Generated macro for iter_after (function)
macro_rules! Depcrate_pathiter_after {
() => {
// Module: crate::path
// Provides: {"iter_after"}
// Dependencies: {}
fn iter_after < 'a , 'b , I , J > (mut iter : I , mut prefix : J) -> Option < I > where I : Iterator < Item = Component < 'a > > + Clone , J : Iterator < Item = Component < 'b > > , { loop { let mut iter_next = iter . clone () ; match (iter_next . next () , prefix . next ()) { (Some (ref x) , Some (ref y)) if x == y => () , (Some (_) , Some (_)) => return None , (Some (_) , None) => return Some (iter) , (None , None) => return Some (iter) , (None , Some (_)) => return None , } iter = iter_next ; } }
};
}
