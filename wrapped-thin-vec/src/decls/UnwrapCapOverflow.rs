macro_rules! UnwrapCapOverflow {
    () => {
        trait UnwrapCapOverflow < T > { fn unwrap_cap_overflow (self) -> T ; }
    };
}

UnwrapCapOverflow!()