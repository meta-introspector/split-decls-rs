macro_rules! level_enabled {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! level_enabled { ($ lvl : expr) => { $ lvl <= $ crate :: level_filters :: STATIC_MAX_LEVEL && $ lvl <= $ crate :: level_filters :: LevelFilter :: current () } ; }
    };
}

level_enabled!()