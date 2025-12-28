macro_rules! deps {
    () => {
        WitnessStack!();
        Matrix!();
        PatCx!();
    };
}

macro_rules! WitnessMatrix {
    () => {
        deps!();
        # [doc = " Represents a set of pattern-tuples that are witnesses of non-exhaustiveness for error"] # [doc = " reporting. This has similar invariants as `Matrix` does."] # [doc = ""] # [doc = " The `WitnessMatrix` returned by [`compute_exhaustiveness_and_usefulness`] obeys the invariant"] # [doc = " that the union of the input `Matrix` and the output `WitnessMatrix` together matches the type"] # [doc = " exhaustively."] # [doc = ""] # [doc = " Just as the `Matrix` starts with a single column, by the end of the algorithm, this has a single"] # [doc = " column, which contains the patterns that are missing for the match to be exhaustive."] # [derive (Debug)] struct WitnessMatrix < Cx : PatCx > (Vec < WitnessStack < Cx > >) ;
    };
}

WitnessMatrix!()