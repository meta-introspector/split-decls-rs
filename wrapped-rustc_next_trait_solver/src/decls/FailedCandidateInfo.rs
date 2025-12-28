macro_rules! FailedCandidateInfo {
    () => {
        # [doc = " This is currently used to track the [CandidateHeadUsages] of all failed `ParamEnv`"] # [doc = " candidates. This is then used to ignore their head usages in case there's another"] # [doc = " always applicable `ParamEnv` candidate. Look at how `param_env_head_usages` is"] # [doc = " used in the code for more details."] # [doc = ""] # [doc = " We could easily extend this to also ignore head usages of other ignored candidates."] # [doc = " However, we currently don't have any tests where this matters and the complexity of"] # [doc = " doing so does not feel worth it for now."] # [derive (Debug)] pub (super) struct FailedCandidateInfo { pub param_env_head_usages : CandidateHeadUsages , }
    };
}

FailedCandidateInfo!();