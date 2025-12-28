macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl From < Box < dyn core :: error :: Error + Send + Sync + 'static > > for Error { fn from (source : Box < dyn core :: error :: Error + Send + Sync + 'static >) -> Error { Self :: from_source (source) } }
    };
}

impl_14!()