macro_rules! deps {
    () => {
        XzDecoder!();
        Error!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < W : AsyncWrite > AsyncWrite for XzDecoder < W > { fn shutdown (& mut self) -> Poll < () , io :: Error > { try_nb ! (self . try_finish ()) ; self . get_mut () . shutdown () } }
    };
}

impl_69!()