macro_rules! deps {
    () => {
        SourceMap!();
    };
}

macro_rules! get_source_map {
    () => {
        deps!();
        pub fn get_source_map () -> Option < Arc < SourceMap > > { with_session_globals (| session_globals | session_globals . source_map . clone ()) }
    };
}

get_source_map!()