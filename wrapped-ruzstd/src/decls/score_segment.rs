macro_rules! deps {
    () => {
        KMer!();
        Context!();
    };
}

macro_rules! score_segment {
    () => {
        deps!();
        # [doc = " Given a segment, compute the score (or usefulness) of that segment against the entire epoch."] # [doc = ""] # [doc = " `score_segment` modifies `ctx.frequencies`."] fn score_segment (ctx : & mut Context , collection_sample : & [u8] , segment : & [u8]) -> usize { let mut segment_score = 0 ; for i in 0 .. (segment . len () - K - 1) { let kmer : & KMer = (& segment [i .. i + K]) . try_into () . expect ("Failed to make kmer") ; if ctx . frequencies . contains_key (kmer) { continue ; } let kmer_score = estimate_frequency (kmer , collection_sample) ; ctx . frequencies . insert (* kmer , kmer_score) ; segment_score += kmer_score ; } segment_score }
    };
}

score_segment!();