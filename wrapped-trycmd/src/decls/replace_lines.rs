macro_rules! replace_lines {
    () => {
        # [doc = " Update an inline snapshot"] fn replace_lines (data : & mut String , line_nums : std :: ops :: Range < usize > , text : & str ,) -> Result < () , crate :: Error > { let mut output_lines = String :: new () ; for (line_num , line) in snapbox :: utils :: LinesWithTerminator :: new (data) . enumerate () . map (| (i , l) | (i + 1 , l)) { if line_num == line_nums . start { output_lines . push_str (text) ; if ! text . is_empty () && ! text . ends_with ('\n') { output_lines . push ('\n') ; } } if ! line_nums . contains (& line_num) { output_lines . push_str (line) ; } } * data = output_lines ; Ok (()) }
    };
}

replace_lines!()