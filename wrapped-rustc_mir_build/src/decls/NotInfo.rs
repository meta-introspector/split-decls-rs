macro_rules! NotInfo {
    () => {
        # [derive (Clone , Copy)] struct NotInfo { # [doc = " When visiting the associated expression as a branch condition, treat this"] # [doc = " enclosing `!` as the branch condition instead."] enclosing_not : ExprId , # [doc = " True if the associated expression is nested within an odd number of `!`"] # [doc = " expressions relative to `enclosing_not` (inclusive of `enclosing_not`)."] is_flipped : bool , }
    };
}

NotInfo!();