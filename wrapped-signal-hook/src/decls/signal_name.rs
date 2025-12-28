macro_rules! signal_name {
    () => {
        # [doc = " Provides a human-readable name of a signal."] # [doc = ""] # [doc = " Note that the name does not have to be known (in case it is some less common, or non-standard"] # [doc = " signal)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use signal_hook::low_level::signal_name;"] # [doc = " assert_eq!(\"SIGKILL\", signal_name(9).unwrap());"] # [doc = " assert!(signal_name(142).is_none());"] # [doc = " ```"] pub fn signal_name (signal : c_int) -> Option < & 'static str > { DETAILS . iter () . find (| d | d . signal == signal) . map (| d | d . name) }
    };
}

signal_name!()