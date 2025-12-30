// Generated macro for clone_arc_borrow (function)
macro_rules! Depcrate_arc_borrowclone_arc_borrow {
() => {
// Module: crate::arc_borrow
// Provides: {"clone_arc_borrow"}
// Dependencies: {}
# [test] fn clone_arc_borrow () { let x = Arc :: new (42) ; let b : ArcBorrow < '_ , i32 > = x . borrow_arc () ; let y = b . clone_arc () ; assert_eq ! (x , y) ; }
};
}
