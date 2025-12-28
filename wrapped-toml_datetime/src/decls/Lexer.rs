macro_rules! Lexer {
    () => {
        # [derive (Copy , Clone)] struct Lexer < 's > { stream : & 's str , }
    };
}

Lexer!();