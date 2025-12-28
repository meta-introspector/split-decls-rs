macro_rules! IntoU128 {
    () => {
        # [allow (dead_code , reason = "Too lazy to cfg-gate these")] trait IntoU128 { fn into_u128 (self) -> u128 ; }
    };
}

IntoU128!()