macro_rules! deps {
    () => {
        Parser!();
        Piece!();
        ParseError!();
        Suggestion!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'input > Iterator for Parser < 'input > { type Item = Piece < 'input > ; fn next (& mut self) -> Option < Piece < 'input > > { if let Some ((Range { start , end } , idx , ch)) = self . peek () { match ch { '{' => { self . input_vec_index += 1 ; if let Some ((_ , i , '{')) = self . peek () { self . input_vec_index += 1 ; Some (Piece :: Lit (self . string (i))) } else { self . last_open_brace = Some (start .. end) ; let arg = self . argument () ; self . ws () ; if let Some ((close_brace_range , _)) = self . consume_pos ('}') { if self . is_source_literal { self . arg_places . push (start .. close_brace_range . end) ; } } else { self . missing_closing_brace (& arg) ; } Some (Piece :: NextArgument (Box :: new (arg))) } } '}' => { self . input_vec_index += 1 ; if let Some ((_ , i , '}')) = self . peek () { self . input_vec_index += 1 ; Some (Piece :: Lit (self . string (i))) } else { self . errors . push (ParseError { description : "unmatched `}` found" . into () , note : Some ("if you intended to print `}`, you can escape it using `}}`" . into () ,) , label : "unmatched `}`" . into () , span : start .. end , secondary_label : None , suggestion : Suggestion :: None , }) ; None } } _ => Some (Piece :: Lit (self . string (idx))) , } } else { if self . is_source_literal { let span = self . cur_line_start .. self . end_of_snippet ; if self . line_spans . last () != Some (& span) { self . line_spans . push (span) ; } } None } } }
    };
}

impl_14!()