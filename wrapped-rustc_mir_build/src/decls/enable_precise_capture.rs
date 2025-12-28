macro_rules! enable_precise_capture {
    () => {
        # [doc = " Precise capture is enabled if user is using Rust Edition 2021 or higher."] fn enable_precise_capture (closure_span : Span) -> bool { closure_span . at_least_rust_2021 () }
    };
}

enable_precise_capture!()