macro_rules! deps {
    () => {
        VersionReq!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        # [doc = " The default VersionReq is the same as [`VersionReq::STAR`]."] impl Default for VersionReq { fn default () -> Self { VersionReq :: STAR } }
    };
}

impl_93!();