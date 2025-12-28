macro_rules! IntoU64 {
    () => {
        # [allow (dead_code , reason = "Too lazy to cfg-gate these")] trait IntoU64 { fn into_u64 (self) -> u64 ; }
    };
}

IntoU64!();