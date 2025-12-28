macro_rules! deps {
    () => {
        PatCx!();
        Matrix!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        # [doc = " Pretty-printer for matrices of patterns, example:"] # [doc = ""] # [doc = " ```text"] # [doc = " + _     + []                +"] # [doc = " + true  + [First]           +"] # [doc = " + true  + [Second(true)]    +"] # [doc = " + false + [_]               +"] # [doc = " + _     + [_, _, tail @ ..] +"] # [doc = " | ✓     | ?                 | // validity"] # [doc = " ```"] impl < 'p , Cx : PatCx > fmt :: Debug for Matrix < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "\n") ? ; let mut pretty_printed_matrix : Vec < Vec < String > > = self . rows . iter () . map (| row | row . iter () . map (| pat | format ! ("{pat:?}")) . collect ()) . collect () ; pretty_printed_matrix . push (self . place_info . iter () . map (| place | format ! ("{}" , place . validity)) . collect ()) ; let column_count = self . column_count () ; assert ! (self . rows . iter () . all (| row | row . len () == column_count)) ; assert ! (self . place_info . len () == column_count) ; let column_widths : Vec < usize > = (0 .. column_count) . map (| col | pretty_printed_matrix . iter () . map (| row | row [col] . len ()) . max () . unwrap_or (0)) . collect () ; for (row_i , row) in pretty_printed_matrix . into_iter () . enumerate () { let is_validity_row = row_i == self . rows . len () ; let sep = if is_validity_row { "|" } else { "+" } ; write ! (f , "{sep}") ? ; for (column , pat_str) in row . into_iter () . enumerate () { write ! (f , " ") ? ; write ! (f , "{:1$}" , pat_str , column_widths [column]) ? ; write ! (f , " {sep}") ? ; } if is_validity_row { write ! (f , " // validity") ? ; } write ! (f , "\n") ? ; } Ok (()) } }
    };
}

impl_112!()