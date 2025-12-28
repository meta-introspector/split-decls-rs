macro_rules! deps {
    () => {
        UsageMap!();
    };
}

macro_rules! SharedState {
    () => {
        deps!();
        # [doc = " The state that is shared across the concurrent threads that are doing collection."] struct SharedState < 'tcx > { # [doc = " Items that have been or are currently being recursively collected."] visited : MTLock < UnordSet < MonoItem < 'tcx > > > , # [doc = " Items that have been or are currently being recursively treated as \"mentioned\", i.e., their"] # [doc = " consts are evaluated but nothing is added to the collection."] mentioned : MTLock < UnordSet < MonoItem < 'tcx > > > , # [doc = " Which items are being used where, for better errors."] usage_map : MTLock < UsageMap < 'tcx > > , }
    };
}

SharedState!();