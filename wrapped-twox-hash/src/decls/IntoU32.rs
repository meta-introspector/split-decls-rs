macro_rules! IntoU32 {
    () => {
        # [allow (dead_code , reason = "Too lazy to cfg-gate these")] trait IntoU32 { fn into_u32 (self) -> u32 ; }
    };
}

IntoU32!()