macro_rules! GoalStalledOn {
    () => {
        # [doc = " The conditions that must change for a goal to warrant"] # [derive_where (Clone , Debug ; I : Interner)] pub struct GoalStalledOn < I : Interner > { pub num_opaques : usize , pub stalled_vars : Vec < I :: GenericArg > , pub sub_roots : Vec < TyVid > , # [doc = " The cause that will be returned on subsequent evaluations if this goal remains stalled."] pub stalled_cause : MaybeCause , }
    };
}

GoalStalledOn!();