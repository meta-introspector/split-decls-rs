macro_rules! State {
    () => {
        struct State { current_prefix : Option < toml_parser :: Span > , current_key : Option < toml_parser :: parser :: Event > , current_suffix : Option < toml_parser :: Span > , }
    };
}

State!();