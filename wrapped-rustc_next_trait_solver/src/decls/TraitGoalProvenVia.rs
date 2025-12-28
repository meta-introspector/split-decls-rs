macro_rules! TraitGoalProvenVia {
    () => {
        # [doc = " How we've proven this trait goal."] # [doc = ""] # [doc = " This is used by `NormalizesTo` goals to only normalize"] # [doc = " by using the same 'kind of candidate' we've used to prove"] # [doc = " its corresponding trait goal. Most notably, we do not"] # [doc = " normalize by using an impl if the trait goal has been"] # [doc = " proven via a `ParamEnv` candidate."] # [doc = ""] # [doc = " This is necessary to avoid unnecessary region constraints,"] # [doc = " see trait-system-refactor-initiative#125 for more details."] # [derive (Debug , Clone , Copy)] pub (super) enum TraitGoalProvenVia { # [doc = " We've proven the trait goal by something which is"] # [doc = " is not a non-global where-bound or an alias-bound."] # [doc = ""] # [doc = " This means we don't disable any candidates during"] # [doc = " normalization."] Misc , ParamEnv , AliasBound , }
    };
}

TraitGoalProvenVia!()