macro_rules! deps {
    () => {
        PatCx!();
        WitnessPat!();
        PatStack!();
    };
}

macro_rules! WitnessStack {
    () => {
        deps!();
        # [doc = " A witness-tuple of non-exhaustiveness for error reporting, represented as a list of patterns (in"] # [doc = " reverse order of construction)."] # [doc = ""] # [doc = " This mirrors `PatStack`: they function similarly, except `PatStack` contains user patterns we"] # [doc = " are inspecting, and `WitnessStack` contains witnesses we are constructing."] # [doc = " FIXME(Nadrieril): use the same order of patterns for both."] # [doc = ""] # [doc = " A `WitnessStack` should have the same types and length as the `PatStack`s we are inspecting"] # [doc = " (except we store the patterns in reverse order). The same way `PatStack` starts with length 1,"] # [doc = " at the end of the algorithm this will have length 1. In the middle of the algorithm, it can"] # [doc = " contain multiple patterns."] # [doc = ""] # [doc = " For example, if we are constructing a witness for the match against"] # [doc = ""] # [doc = " ```compile_fail,E0004"] # [doc = " struct Pair(Option<(u32, u32)>, bool);"] # [doc = " # fn foo(p: Pair) {"] # [doc = " match p {"] # [doc = "    Pair(None, _) => {}"] # [doc = "    Pair(_, false) => {}"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " We'll perform the following steps (among others):"] # [doc = " ```text"] # [doc = " - Start with a matrix representing the match"] # [doc = "     `PatStack(vec![Pair(None, _)])`"] # [doc = "     `PatStack(vec![Pair(_, false)])`"] # [doc = " - Specialize with `Pair`"] # [doc = "     `PatStack(vec![None, _])`"] # [doc = "     `PatStack(vec![_, false])`"] # [doc = " - Specialize with `Some`"] # [doc = "     `PatStack(vec![_, false])`"] # [doc = " - Specialize with `_`"] # [doc = "     `PatStack(vec![false])`"] # [doc = " - Specialize with `true`"] # [doc = "     // no patstacks left"] # [doc = " - This is a non-exhaustive match: we have the empty witness stack as a witness."] # [doc = "     `WitnessStack(vec![])`"] # [doc = " - Apply `true`"] # [doc = "     `WitnessStack(vec![true])`"] # [doc = " - Apply `_`"] # [doc = "     `WitnessStack(vec![true, _])`"] # [doc = " - Apply `Some`"] # [doc = "     `WitnessStack(vec![true, Some(_)])`"] # [doc = " - Apply `Pair`"] # [doc = "     `WitnessStack(vec![Pair(Some(_), true)])`"] # [doc = " ```"] # [doc = ""] # [doc = " The final `Pair(Some(_), true)` is then the resulting witness."] # [doc = ""] # [doc = " See the top of the file for more detailed explanations and examples."] # [derive (Debug)] struct WitnessStack < Cx : PatCx > (Vec < WitnessPat < Cx > >) ;
    };
}

WitnessStack!()