macro_rules! deps {
    () => {
        AllowedOptions!();
    };
}

macro_rules! SalsaStructAllowedOptions {
    () => {
        deps!();
        pub (crate) trait SalsaStructAllowedOptions : AllowedOptions { # [doc = " The kind of struct (e.g., interned, input, tracked)."] const KIND : & 'static str ; # [doc = " Are `#[maybe_update]` fields allowed?"] const ALLOW_MAYBE_UPDATE : bool ; # [doc = " Are `#[tracked]` fields allowed?"] const ALLOW_TRACKED : bool ; # [doc = " Does this kind of struct have a `'db` lifetime?"] const HAS_LIFETIME : bool ; # [doc = " Can this struct elide the `'db` lifetime?"] const ELIDABLE_LIFETIME : bool ; # [doc = " Are `#[default]` fields allowed?"] const ALLOW_DEFAULT : bool ; }
    };
}

SalsaStructAllowedOptions!()