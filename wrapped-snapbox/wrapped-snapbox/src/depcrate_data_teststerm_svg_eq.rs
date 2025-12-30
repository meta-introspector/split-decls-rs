// Generated macro for term_svg_eq (function)
macro_rules! Depcrate_data_teststerm_svg_eq {
() => {
// Module: crate::data::tests
// Provides: {"term_svg_eq"}
// Dependencies: {}
# [test] # [cfg (feature = "term-svg")] fn term_svg_eq () { let left = Data :: with_inner (DataInner :: TermSvg ("
irrelevant
  <text>relevant

</text>
irrelevant" . to_owned () ,)) ; let right = Data :: with_inner (DataInner :: TermSvg ("
irrelevant
  <text>relevant

</text>
irrelevant" . to_owned () ,)) ; assert_eq ! (left , right) ; let left = Data :: with_inner (DataInner :: TermSvg ("
irrelevant 1
  <text>relevant

</text>
irrelevant 1" . to_owned () ,)) ; let right = Data :: with_inner (DataInner :: TermSvg ("
irrelevant 2
  <text>relevant

</text>
irrelevant 2" . to_owned () ,)) ; assert_eq ! (left , right) ; }
};
}
