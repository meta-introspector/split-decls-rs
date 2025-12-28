macro_rules! deps {
    () => {
        VerboseTimingGuard!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl Drop for VerboseTimingGuard < '_ > { fn drop (& mut self) { if let Some (info) = & self . info { let end_rss = get_resident_set_size () ; let dur = info . start_time . elapsed () ; print_time_passes_entry (& info . message , dur , info . start_rss , end_rss , info . format) ; } } }
    };
}

impl_377!();