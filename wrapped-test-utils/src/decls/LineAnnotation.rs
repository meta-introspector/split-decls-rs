macro_rules! LineAnnotation {
    () => {
        enum LineAnnotation { Annotation { range : TextRange , content : String , file : bool } , Continuation { offset : TextSize , content : String } , }
    };
}

LineAnnotation!();