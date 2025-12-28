macro_rules! deps {
    () => {
        AsDisplay!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < 'a > AsDisplay < 'a > for PathBuf { type Target = path :: Display < 'a > ; # [inline] fn as_display (& 'a self) -> Self :: Target { self . display () } }
    };
}

impl_19!()