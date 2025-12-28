macro_rules! SpeculationFeature {
    () => {
        # [doc = " `PR_SPEC_*` values for use with [`speculative_feature_state`] and"] # [doc = " [`control_speculative_feature`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum SpeculationFeature { # [doc = " Set the state of the speculative store bypass misfeature."] SpeculativeStoreBypass = PR_SPEC_STORE_BYPASS , # [doc = " Set the state of the indirect branch speculation misfeature."] IndirectBranchSpeculation = PR_SPEC_INDIRECT_BRANCH , # [doc = " Flush L1D Cache on context switch out of the task."] FlushL1DCacheOnContextSwitchOutOfTask = PR_SPEC_L1D_FLUSH , }
    };
}

SpeculationFeature!()