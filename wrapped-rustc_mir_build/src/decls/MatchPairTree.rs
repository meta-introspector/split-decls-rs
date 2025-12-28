macro_rules! deps {
    () => {
        TestCase!();
    };
}

macro_rules! MatchPairTree {
    () => {
        deps!();
        # [doc = " Node in a tree of \"match pairs\", where each pair consists of a place to be"] # [doc = " tested, and a test to perform on that place."] # [doc = ""] # [doc = " Each node also has a list of subpairs (possibly empty) that must also match,"] # [doc = " and a reference to the THIR pattern it represents."] # [derive (Debug , Clone)] pub (crate) struct MatchPairTree < 'tcx > { # [doc = " This place..."] # [doc = ""] # [doc = " ---"] # [doc = " This can be `None` if it referred to a non-captured place in a closure."] # [doc = ""] # [doc = " Invariant: Can only be `None` when `test_case` is `Or`."] # [doc = " Therefore this must be `Some(_)` after or-pattern expansion."] place : Option < Place < 'tcx > > , # [doc = " ... must pass this test..."] test_case : TestCase < 'tcx > , # [doc = " ... and these subpairs must match."] # [doc = ""] # [doc = " ---"] # [doc = " Subpairs typically represent tests that can only be performed after their"] # [doc = " parent has succeeded. For example, the pattern `Some(3)` might have an"] # [doc = " outer match pair that tests for the variant `Some`, and then a subpair"] # [doc = " that tests its field for the value `3`."] subpairs : Vec < Self > , # [doc = " Type field of the pattern this node was created from."] pattern_ty : Ty < 'tcx > , # [doc = " Span field of the pattern this node was created from."] pattern_span : Span , }
    };
}

MatchPairTree!();