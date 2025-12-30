// Generated macro for tests (module)
macro_rules! Depcrate_renderer_square_bracketstests {
() => {
// Module: crate::renderer::square_brackets
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn can_pull_out_square_bracket () { assert_eq ! (pull_out_square_bracket ("hi") , Vec ::< String >:: new ()) ; assert_eq ! (pull_out_square_bracket ("['hi']") , Vec ::< String >:: new ()) ; assert_eq ! (pull_out_square_bracket ("[hi] a[0]") , vec ! ["hi"]) ; assert_eq ! (pull_out_square_bracket ("hi [th[e]['r']e] [fish]") , vec ! ["th[e]['r']e" , "fish"]) ; } }
};
}
