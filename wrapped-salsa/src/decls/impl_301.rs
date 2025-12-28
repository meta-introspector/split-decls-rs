macro_rules! deps {
    () => {
        Coordinate!();
        CoordinateDrop!();
    };
}

macro_rules! impl_301 {
    () => {
        deps!();
        impl std :: ops :: Deref for CoordinateDrop { type Target = Arc < Coordinate > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_301!()