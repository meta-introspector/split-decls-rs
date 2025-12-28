macro_rules! KeyMetrics {
    () => {
        # [derive (Copy , Clone , Debug)] struct KeyMetrics { unquoted : bool , single_quotes : bool , double_quotes : bool , escape_codes : bool , escape : bool , }
    };
}

KeyMetrics!();