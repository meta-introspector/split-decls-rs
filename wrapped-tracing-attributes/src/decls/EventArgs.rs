macro_rules! deps {
    () => {
        Level!();
        FormatMode!();
    };
}

macro_rules! EventArgs {
    () => {
        deps!();
        # [doc = " Arguments to `#[instrument(err(...))]` and `#[instrument(ret(...))]` which describe how the"] # [doc = " return value event should be emitted."] # [derive (Clone , Default , Debug)] pub (crate) struct EventArgs { level : Option < Level > , pub (crate) mode : FormatMode , }
    };
}

EventArgs!()