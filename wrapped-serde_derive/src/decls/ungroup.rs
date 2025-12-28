macro_rules! ungroup {
    () => {
        pub fn ungroup (mut ty : & Type) -> & Type { while let Type :: Group (group) = ty { ty = & group . elem ; } ty }
    };
}

ungroup!();