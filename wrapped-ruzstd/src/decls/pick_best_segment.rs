macro_rules! deps {
    () => {
        Context!();
        DictParams!();
        Segment!();
    };
}

macro_rules! pick_best_segment {
    () => {
        deps!();
        # [doc = " Returns the highest scoring segment in an epoch"] # [doc = " as a slice of that epoch."] pub fn pick_best_segment (params : & DictParams , ctx : & mut Context , collection_sample : & '_ [u8] ,) -> Segment { let mut segments = collection_sample . chunks (params . segment_size as usize) . peekable () ; let mut best_segment : & [u8] = segments . peek () . expect ("at least one segment") ; let mut top_segment_score : usize = 0 ; for segment in segments { let segment_score = score_segment (ctx , collection_sample , segment) ; if segment_score > top_segment_score { best_segment = segment ; top_segment_score = segment_score ; } } Segment { raw : best_segment . into () , score : top_segment_score , } }
    };
}

pick_best_segment!()