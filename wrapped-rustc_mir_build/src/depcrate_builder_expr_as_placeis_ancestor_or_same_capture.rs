// Generated macro for is_ancestor_or_same_capture (function)
macro_rules! Depcrate_builder_expr_as_placeis_ancestor_or_same_capture {
() => {
// Module: crate::builder::expr::as_place
// Provides: {"is_ancestor_or_same_capture"}
// Dependencies: {}
# [doc = " Return true if the `proj_possible_ancestor` represents an ancestor path"] # [doc = " to `proj_capture` or `proj_possible_ancestor` is same as `proj_capture`,"] # [doc = " assuming they both start off of the same root variable."] # [doc = ""] # [doc = " **Note:** It's the caller's responsibility to ensure that both lists of projections"] # [doc = "           start off of the same root variable."] # [doc = ""] # [doc = " Eg: 1. `foo.x` which is represented using `projections=[Field(x)]` is an ancestor of"] # [doc = "        `foo.x.y` which is represented using `projections=[Field(x), Field(y)]`."] # [doc = "        Note both `foo.x` and `foo.x.y` start off of the same root variable `foo`."] # [doc = "     2. Since we only look at the projections here function will return `bar.x` as a valid"] # [doc = "        ancestor of `foo.x.y`. It's the caller's responsibility to ensure that both projections"] # [doc = "        list are being applied to the same root variable."] fn is_ancestor_or_same_capture (proj_possible_ancestor : & [HirProjectionKind] , proj_capture : & [HirProjectionKind] ,) -> bool { if proj_possible_ancestor . len () > proj_capture . len () { return false ; } iter :: zip (proj_possible_ancestor , proj_capture) . all (| (a , b) | a == b) }
};
}
