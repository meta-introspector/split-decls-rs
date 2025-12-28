macro_rules! KMer {
    () => {
        # [doc = "As found under \"4: Experiments - Varying k-mer Size\" in the original paper,"] # [doc = " \"when k = 16, across all our text collections, there is a reasonable spread\""] # [doc = ""] # [doc = " Reasonable range: [6, 16]"] pub (super) type KMer = [u8 ; K] ;
    };
}

KMer!()