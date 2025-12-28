macro_rules! deps {
    () => {
        BridgeTys!();
        Allocation!();
    };
}

macro_rules! allocation_filter {
    () => {
        deps!();
        # [doc = " Creates an `Allocation` only from information within the `AllocRange`."] pub (super) fn allocation_filter < 'tcx > (alloc : & rustc_middle :: mir :: interpret :: Allocation , alloc_range : AllocRange , tables : & mut Tables < 'tcx , BridgeTys > , cx : & CompilerCtxt < 'tcx , BridgeTys > ,) -> Allocation { alloc :: allocation_filter (alloc , alloc_range , tables , cx) }
    };
}

allocation_filter!()