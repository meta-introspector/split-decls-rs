macro_rules! MaybeCause {
    () => {
        # [doc = " Why we failed to evaluate a goal."] # [derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub enum MaybeCause { # [doc = " We failed due to ambiguity. This ambiguity can either"] # [doc = " be a true ambiguity, i.e. there are multiple different answers,"] # [doc = " or we hit a case where we just don't bother, e.g. `?x: Trait` goals."] Ambiguity , # [doc = " We gave up due to an overflow, most often by hitting the recursion limit."] Overflow { suggest_increasing_limit : bool , keep_constraints : bool } , }
    };
}

MaybeCause!()