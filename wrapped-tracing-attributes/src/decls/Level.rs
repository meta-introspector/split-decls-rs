macro_rules! Level {
    () => {
        # [derive (Clone , Debug)] pub (crate) enum Level { Trace , Debug , Info , Warn , Error , Path (Path) , }
    };
}

Level!()