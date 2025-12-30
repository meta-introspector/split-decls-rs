// Generated macro for test_unified_diff (function)
macro_rules! Depcrate_udifftest_unified_diff {
() => {
// Module: crate::udiff
// Provides: {"test_unified_diff"}
// Dependencies: {}
# [test] fn test_unified_diff () { let diff = TextDiff :: from_lines ("a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\nm\nn\no\np\nq\nr\ns\nt\nu\nv\nw\nx\ny\nz\nA\nB\nC\nD\nE\nF\nG\nH\nI\nJ\nK\nL\nM\nN\nO\nP\nQ\nR\nS\nT\nU\nV\nW\nX\nY\nZ" , "a\nb\nc\nd\ne\nf\ng\nh\ni\nj\nk\nl\nm\nn\no\np\nq\nr\nS\nt\nu\nv\nw\nx\ny\nz\nA\nB\nC\nD\nE\nF\nG\nH\nI\nJ\nK\nL\nM\nN\no\nP\nQ\nR\nS\nT\nU\nV\nW\nX\nY\nZ" ,) ; insta :: assert_snapshot ! (& diff . unified_diff () . header ("a.txt" , "b.txt") . to_string ()) ; }
};
}
