macro_rules! deps {
    () => {
        DebugWithContext!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < C > DebugWithContext < C > for crate :: move_paths :: InitIndex { }
    };
}

impl_53!()