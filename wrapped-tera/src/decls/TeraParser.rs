macro_rules! TeraParser {
    () => {
        # [derive (Parser)] # [grammar = "parser/tera.pest"] pub struct TeraParser ;
    };
}

TeraParser!();