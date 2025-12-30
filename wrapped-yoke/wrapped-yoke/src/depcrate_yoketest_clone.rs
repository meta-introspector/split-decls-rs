// Generated macro for test_clone (function)
macro_rules! Depcrate_yoketest_clone {
() => {
// Module: crate::yoke
// Provides: {"test_clone"}
// Dependencies: {}
# [test] fn test_clone () { let local_data = "foo" . to_owned () ; let y1 = Yoke :: < alloc :: borrow :: Cow < 'static , str > , Rc < String > > :: attach_to_zero_copy_cart (Rc :: new (local_data) ,) ; let y2 = y1 . clone () ; assert_eq ! (y1 . get () , "foo") ; assert_eq ! (y2 . get () , "foo") ; let mut y3 = y1 . clone () ; y3 . with_mut (| y | { y . to_mut () . push_str ("bar") ; }) ; assert_eq ! (y1 . get () , "foo") ; assert_eq ! (y2 . get () , "foo") ; assert_eq ! (y3 . get () , "foobar") ; let y4 = y3 . clone () ; y3 . with_mut (| y | { y . to_mut () . push_str ("baz") ; }) ; assert_eq ! (y1 . get () , "foo") ; assert_eq ! (y2 . get () , "foo") ; assert_eq ! (y3 . get () , "foobarbaz") ; assert_eq ! (y4 . get () , "foobar") ; }
};
}
