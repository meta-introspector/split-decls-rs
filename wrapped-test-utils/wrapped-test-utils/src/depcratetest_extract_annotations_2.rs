// Generated macro for test_extract_annotations_2 (function)
macro_rules! Depcratetest_extract_annotations_2 {
() => {
// Module: crate
// Provides: {"test_extract_annotations_2"}
// Dependencies: {}
# [test] fn test_extract_annotations_2 () { let text = stdx :: trim_indent (r#"
fn main() {
    (x,   y);
   //^ a
      //  ^ b
  //^^^^^^^^ c
}"# ,) ; let res = extract_annotations (& text) . into_iter () . map (| (range , ann) | (& text [range] , ann)) . collect :: < Vec < _ > > () ; assert_eq ! (res , [("x" , "a" . into ()) , ("y" , "b" . into ()) , ("(x,   y)" , "c" . into ())]) ; }
};
}
