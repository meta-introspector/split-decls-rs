macro_rules! deps {
    () => {
        SpeculationFeature!();
        Result!();
    };
}

macro_rules! impl_1023 {
    () => {
        deps!();
        impl TryFrom < u32 > for SpeculationFeature { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_SPEC_STORE_BYPASS => Ok (Self :: SpeculativeStoreBypass) , PR_SPEC_INDIRECT_BRANCH => Ok (Self :: IndirectBranchSpeculation) , PR_SPEC_L1D_FLUSH => Ok (Self :: FlushL1DCacheOnContextSwitchOutOfTask) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_1023!()