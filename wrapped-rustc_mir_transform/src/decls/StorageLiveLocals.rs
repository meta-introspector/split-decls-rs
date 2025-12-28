macro_rules! StorageLiveLocals {
    () => {
        # [derive (Debug)] pub (crate) struct StorageLiveLocals { # [doc = " Set of \"StorageLive\" statements for each local."] storage_live : IndexVec < Local , Set1 < DefLocation > > , }
    };
}

StorageLiveLocals!();