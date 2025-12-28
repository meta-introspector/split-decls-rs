macro_rules! deps {
    () => {
        Key!();
        State!();
    };
}

macro_rules! try_cache {
    () => {
        deps!();
        pub (crate) fn try_cache (metadata : & Metadata < '_ > , callback : impl FnOnce () -> bool) -> bool { STATE . with (| state | { let mut state = state . borrow_mut () ; let epoch = interest_cache_epoch () ; if epoch != state . epoch { * state = State :: new (epoch , & CONFIG . lock () . unwrap ()) ; } let level = metadata . level () ; if state . cache . cap () == 0 || level < state . min_verbosity { return callback () ; } let target = metadata . target () ; let mut hasher = AHasher :: default () ; hasher . write (target . as_bytes ()) ; const HASH_MASK : u64 = ! 1 ; const INTEREST_MASK : u64 = 1 ; let target_hash = hasher . finish () & HASH_MASK ; let key = Key { target_address : target . as_ptr () as usize , level_and_length : level as usize | target . len () . wrapping_shl (3) , } ; if let Some (& cached) = state . cache . get (& key) { if cached & HASH_MASK == target_hash { return (cached & INTEREST_MASK) != 0 ; } } let interest = callback () ; state . cache . put (key , target_hash | interest as u64) ; interest }) }
    };
}

try_cache!();