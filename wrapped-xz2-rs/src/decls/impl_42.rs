macro_rules! deps {
    () => {
        Error!();
        XzDecoder!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncWrite > AsyncWrite for XzDecoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
    };
}

impl_42!()