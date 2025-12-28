macro_rules! deps {
    () => {
        MetavarSpansMap!();
    };
}

macro_rules! with_metavar_spans {
    () => {
        deps!();
        # [inline] pub fn with_metavar_spans < R > (f : impl FnOnce (& MetavarSpansMap) -> R) -> R { with_session_globals (| session_globals | f (& session_globals . metavar_spans)) }
    };
}

with_metavar_spans!();