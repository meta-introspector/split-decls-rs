macro_rules! ScheduleDrops {
    () => {
        # [doc = " Used by [`Builder::storage_live_binding`] and [`Builder::bind_matched_candidate_for_arm_body`]"] # [doc = " to decide whether to schedule drops."] # [derive (Clone , Copy , Debug)] pub (crate) enum ScheduleDrops { # [doc = " Yes, the relevant functions should also schedule drops as appropriate."] Yes , # [doc = " No, don't schedule drops. The caller has taken responsibility for any"] # [doc = " appropriate drops."] No , }
    };
}

ScheduleDrops!()