macro_rules! deps {
    () => {
        RawUid!();
        RawGid!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn test_sizes () { assert_eq_size ! (RawUid , u32) ; assert_eq_size ! (RawGid , u32) ; assert_eq_size ! (RawUid , libc :: uid_t) ; assert_eq_size ! (RawGid , libc :: gid_t) ; } }
    };
}

tests!();