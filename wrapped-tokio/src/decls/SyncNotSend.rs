macro_rules! SyncNotSend {
    () => {
        # [doc = " Marker for types that are `Sync` but not `Send`"] # [allow (dead_code)] pub (crate) struct SyncNotSend (# [allow (dead_code)] * mut ()) ;
    };
}

SyncNotSend!()