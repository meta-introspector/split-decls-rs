macro_rules! deps {
    () => {
        MockTask!();
    };
}

macro_rules! Spawn {
    () => {
        deps!();
        # [doc = " Future spawned on a mock task that can be used to poll the future or stream"] # [doc = " without needing pinning or context types."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct Spawn < T > { task : MockTask , future : Pin < Box < T > > , }
    };
}

Spawn!()