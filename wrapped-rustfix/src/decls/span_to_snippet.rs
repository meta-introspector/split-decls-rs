macro_rules! deps {
    () => {
        DiagnosticSpan!();
        Snippet!();
        LinePosition!();
        LineRange!();
    };
}

macro_rules! span_to_snippet {
    () => {
        deps!();
        # [doc = " Converts a [`DiagnosticSpan`] to a [`Snippet`]."] fn span_to_snippet (span : & DiagnosticSpan) -> Snippet { Snippet { file_name : span . file_name . clone () , line_range : LineRange { start : LinePosition { line : span . line_start , column : span . column_start , } , end : LinePosition { line : span . line_end , column : span . column_end , } , } , range : (span . byte_start as usize) .. (span . byte_end as usize) , } }
    };
}

span_to_snippet!()