// Generated macro for term_svg_ne (function)
macro_rules! Depcrate_data_teststerm_svg_ne {
() => {
// Module: crate::data::tests
// Provides: {"term_svg_ne"}
// Dependencies: {}
# [test] # [cfg (feature = "term-svg")] fn term_svg_ne () { let left = Data :: with_inner (DataInner :: TermSvg ("
irrelevant 1
  <text>relevant 1

</text>
irrelevant 1" . to_owned () ,)) ; let right = Data :: with_inner (DataInner :: TermSvg ("
irrelevant 2
  <text>relevant 2

</text>
irrelevant 2" . to_owned () ,)) ; assert_ne ! (left , right) ; }
};
}
