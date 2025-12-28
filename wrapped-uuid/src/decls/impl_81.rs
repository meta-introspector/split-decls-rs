macro_rules! deps {
    () => {
        Timestamp!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < Timestamp > for std :: time :: SystemTime { fn from (ts : Timestamp) -> Self { let (seconds , subsec_nanos) = ts . to_unix () ; Self :: UNIX_EPOCH + std :: time :: Duration :: new (seconds , subsec_nanos) } }
    };
}

impl_81!();