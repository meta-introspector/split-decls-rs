macro_rules! deps {
    () => {
        PatCx!();
        DeconstructedPat!();
    };
}

macro_rules! BranchPatUsefulness {
    () => {
        deps!();
        # [doc = " A pattern is a \"branch\" if it is the immediate child of an or-pattern, or if it is the whole"] # [doc = " pattern of a match arm. These are the patterns that can be meaningfully considered \"redundant\","] # [doc = " since e.g. `0` in `(0, 1)` cannot be redundant on its own."] # [doc = ""] # [doc = " We track for each branch pattern whether it is useful, and if not why."] struct BranchPatUsefulness < 'p , Cx : PatCx > { # [doc = " Whether this pattern is useful."] useful : bool , # [doc = " A set of patterns that:"] # [doc = " - come before this one in the match;"] # [doc = " - intersect this one;"] # [doc = " - at the end of the algorithm, if `!self.useful`, their union covers this pattern."] covered_by : FxHashSet < & 'p DeconstructedPat < Cx > > , }
    };
}

BranchPatUsefulness!();