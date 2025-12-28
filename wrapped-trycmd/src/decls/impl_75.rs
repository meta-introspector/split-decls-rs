macro_rules! deps {
    () => {
        FileStatus!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl FileStatus { fn is_ok (& self) -> bool { match self { Self :: Ok { .. } => true , Self :: Failure (_) | Self :: TypeMismatch { .. } | Self :: LinkMismatch { .. } | Self :: ContentMismatch { .. } => false , } } }
    };
}

impl_75!();