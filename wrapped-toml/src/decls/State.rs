macro_rules! State {
    () => {
        struct State { current_key : Option < toml_parser :: parser :: Event > , }
    };
}

State!();