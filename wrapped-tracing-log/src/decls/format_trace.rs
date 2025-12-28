macro_rules! format_trace {
    () => {
        # [doc = " Format a log record as a trace event in the current span."] pub fn format_trace (record : & log :: Record < '_ >) -> io :: Result < () > { dispatch_record (record) ; Ok (()) }
    };
}

format_trace!()