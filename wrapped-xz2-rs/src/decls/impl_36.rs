macro_rules! deps {
    () => {
        Error!();
        XzEncoder!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncWrite > AsyncWrite for XzEncoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
    };
}

impl_36!();