macro_rules! deps {
    () => {
        Stability!();
        ImpliedFeatures!();
    };
}

macro_rules! BPF_FEATURES {
    () => {
        deps!();
        const BPF_FEATURES : & [(& str , Stability , ImpliedFeatures)] = & [("alu32" , Unstable (sym :: bpf_target_feature) , & [])] ;
    };
}

BPF_FEATURES!()