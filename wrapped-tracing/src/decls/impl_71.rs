macro_rules! deps {
    () => {
        Span!();
        Inner!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Drop for Span { # [inline (always)] fn drop (& mut self) { if let Some (Inner { ref id , ref subscriber , }) = self . inner { subscriber . try_close (id . clone ()) ; } if_log_enabled ! { crate :: Level :: TRACE , { if let Some (meta) = self . meta { self . log (LIFECYCLE_LOG_TARGET , log :: Level :: Trace , format_args ! ("-- {};" , meta . name ()) ,) ; } } } } }
    };
}

impl_71!()