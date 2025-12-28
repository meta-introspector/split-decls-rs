macro_rules! UsageMap {
    () => {
        pub (crate) struct UsageMap < 'tcx > { pub used_map : UnordMap < MonoItem < 'tcx > , Vec < MonoItem < 'tcx > > > , user_map : UnordMap < MonoItem < 'tcx > , Vec < MonoItem < 'tcx > > > , }
    };
}

UsageMap!();