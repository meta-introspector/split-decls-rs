macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! ToSmolStr {
    () => {
        deps!();
        # [doc = " Convert value to [`SmolStr`] using [`fmt::Display`], potentially without allocating."] # [doc = ""] # [doc = " Almost identical to [`ToString`], but converts to `SmolStr` instead."] pub trait ToSmolStr { fn to_smolstr (& self) -> SmolStr ; }
    };
}

ToSmolStr!();