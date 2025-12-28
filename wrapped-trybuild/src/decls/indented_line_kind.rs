macro_rules! deps {
    () => {
        IndentedLineKind!();
    };
}

macro_rules! indented_line_kind {
    () => {
        deps!();
        fn indented_line_kind (line : & str , first_line_in_block : bool , previous_line_is_note : & mut bool , normalization : Normalization ,) -> IndentedLineKind { let previous_line_was_note = mem :: replace (previous_line_is_note , false) ; if let Some (heading_len) = if line . starts_with ("error") { Some ("error" . len ()) } else if line . starts_with ("warning") { Some ("warning" . len ()) } else { None } { if line [heading_len ..] . starts_with (& [':' , '['] [..]) { return IndentedLineKind :: Heading ; } } if first_line_in_block && normalization >= HeadingNote && line . starts_with ("note: ") { return IndentedLineKind :: Heading ; } if line . starts_with ("note:") || line == "..." || normalization >= UnindentAfterHelp && line . starts_with ("help:") || normalization >= UnindentMultilineNote && previous_line_was_note && line . starts_with ("      ") { * previous_line_is_note = true ; return IndentedLineKind :: Note ; } let is_space = | b : & u8 | * b == b' ' ; if let Some (rest) = line . strip_prefix ("... ") { let spaces = rest . bytes () . take_while (is_space) . count () ; return IndentedLineKind :: Code (spaces) ; } let mut spaces = line . bytes () . take_while (is_space) . count () ; let digits = line [spaces ..] . bytes () . take_while (u8 :: is_ascii_digit) . count () ; spaces += line [spaces + digits ..] . bytes () . take_while (is_space) . count () ; let rest = & line [digits + spaces ..] ; if spaces > 0 && (rest == "|" || rest . starts_with ("| ") || normalization >= UnindentSuggestion && digits > 0 && (rest == "~" || rest . starts_with ("~ ") || rest == "+" || rest . starts_with ("+ ") || rest == "-" || rest . starts_with ("- ")) || digits == 0 && (rest . starts_with ("--> ") || rest . starts_with ("::: ") || rest . starts_with ("= "))) { return IndentedLineKind :: Code (spaces - 1) ; } IndentedLineKind :: Other (if digits == 0 { spaces } else { 0 }) }
    };
}

indented_line_kind!();