macro_rules! deps {
    () => {
        IdentityMap!();
        Id!();
        Identity!();
    };
}

macro_rules! TrackedEntry {
    () => {
        deps!();
        # [doc = " A tracked struct entry stored in an [`IdentityMap`]."] # [derive (Debug)] struct TrackedEntry { # [doc = " The identity of the tracked struct."] identity : Identity , # [doc = " The current ID of the tracked struct."] id : Id , # [doc = " Whether or not this tracked struct was created by the current query."] # [doc = ""] # [doc = " Entries where `active` is `false` represent tracked structs that were created"] # [doc = " by a previous execution of the query, but not in the current one, and hence can"] # [doc = " be collected."] active : bool , }
    };
}

TrackedEntry!()