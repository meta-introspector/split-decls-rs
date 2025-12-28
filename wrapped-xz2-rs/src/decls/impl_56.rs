macro_rules! deps {
    () => {
        Error!();
        XzDecoder!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncWrite + Read > AsyncWrite for XzDecoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
    };
}

impl_56!()