macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! on {
    () => {
        deps!();
        # [doc = " Returns true if `stream` is a TTY or the current terminal"] # [doc = " [supports_unicode]."] pub fn on (stream : Stream) -> bool { if ! is_a_tty (stream) { true } else { supports_unicode () } }
    };
}

on!()