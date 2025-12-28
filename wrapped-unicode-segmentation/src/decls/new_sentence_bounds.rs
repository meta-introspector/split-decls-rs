macro_rules! deps {
    () => {
        USentenceBounds!();
    };
}

macro_rules! new_sentence_bounds {
    () => {
        deps!();
        # [inline] pub fn new_sentence_bounds (source : & str) -> USentenceBounds < '_ > { USentenceBounds { iter : fwd :: new_sentence_breaks (source) , sentence_start : None , } }
    };
}

new_sentence_bounds!()