macro_rules! deps {
    () => {
        RevealedTy!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl < 'tcx > std :: ops :: Deref for RevealedTy < 'tcx > { type Target = Ty < 'tcx > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
    };
}

impl_75!()