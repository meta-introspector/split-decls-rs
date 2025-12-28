macro_rules! deps {
    () => {
        UsageMap!();
    };
}

macro_rules! PartitioningCx {
    () => {
        deps!();
        struct PartitioningCx < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , usage_map : & 'a UsageMap < 'tcx > , }
    };
}

PartitioningCx!();