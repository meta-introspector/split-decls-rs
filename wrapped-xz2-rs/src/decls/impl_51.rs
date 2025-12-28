macro_rules! deps {
    () => {
        Error!();
        XzEncoder!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        # [cfg (feature = "tokio")] impl < R : AsyncWrite + Read > AsyncWrite for XzEncoder < R > { fn shutdown (& mut self) -> Poll < () , io :: Error > { self . get_mut () . shutdown () } }
    };
}

impl_51!();