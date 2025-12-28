macro_rules! deps {
    () => {
        IndentedLineKind!();
    };
}

macro_rules! unindent {
    () => {
        deps!();
        fn unindent (diag : String , normalization : Normalization) -> String { if normalization < Unindent { return diag ; } let mut normalized = String :: new () ; let mut lines = diag . lines () ; while let Some (line) = lines . next () { normalized . push_str (line) ; normalized . push ('\n') ; if indented_line_kind (line , true , & mut false , normalization) != IndentedLineKind :: Heading { continue ; } let mut ahead = lines . clone () ; let Some (next_line) = ahead . next () else { continue ; } ; if let IndentedLineKind :: Code (indent) = indented_line_kind (next_line , false , & mut false , normalization) { if next_line [indent + 1 ..] . starts_with ("--> ") { let mut lines_in_block = 1 ; let mut least_indent = indent ; let mut previous_line_is_note = false ; while let Some (line) = ahead . next () { match indented_line_kind (line , false , & mut previous_line_is_note , normalization) { IndentedLineKind :: Heading => break , IndentedLineKind :: Code (indent) => { lines_in_block += 1 ; least_indent = cmp :: min (least_indent , indent) ; } IndentedLineKind :: Note => lines_in_block += 1 , IndentedLineKind :: Other (spaces) => { if spaces > 10 { lines_in_block += 1 ; } else { break ; } } } } previous_line_is_note = false ; for _ in 0 .. lines_in_block { let line = lines . next () . unwrap () ; if let IndentedLineKind :: Code (_) | IndentedLineKind :: Other (_) = indented_line_kind (line , false , & mut previous_line_is_note , normalization) { let space = line . find (' ') . unwrap () ; normalized . push_str (& line [.. space]) ; normalized . push_str (& line [space + least_indent ..]) ; } else { normalized . push_str (line) ; } normalized . push ('\n') ; } } } } normalized }
    };
}

unindent!();