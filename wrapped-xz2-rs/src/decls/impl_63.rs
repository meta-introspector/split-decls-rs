macro_rules! deps {
    () => {
        Error!();
        XzEncoder!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < W : AsyncWrite > AsyncWrite for XzEncoder < W > { fn shutdown (& mut self) -> Poll < () , io :: Error > { try_nb ! (self . try_finish ()) ; self . get_mut () . shutdown () } }
    };
}

impl_63!();