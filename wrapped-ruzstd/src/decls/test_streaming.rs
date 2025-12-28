macro_rules! deps {
    () => {
        StreamingDecoder!();
        Read!();
    };
}

macro_rules! test_streaming {
    () => {
        deps!();
        # [test] # [cfg (feature = "std")] fn test_streaming () { use std :: fs ; use std :: io :: BufReader ; use std :: io :: Read ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; let mut stream = crate :: decoding :: StreamingDecoder :: new (& mut content) . unwrap () ; let mut result = Vec :: new () ; Read :: read_to_end (& mut stream , & mut result) . unwrap () ; let original_f = BufReader :: new (fs :: File :: open ("./decodecorpus_files/z000088") . unwrap ()) ; let original : Vec < u8 > = original_f . bytes () . map (| x | x . unwrap ()) . collect () ; if original . len () != result . len () { panic ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { panic ! ("Result differs in at least {} bytes from original" , counter) ; } let mut content = fs :: File :: open ("./decodecorpus_files/z000068.zst") . unwrap () ; let mut stream = crate :: decoding :: StreamingDecoder :: new_with_decoder (& mut content , stream . into_frame_decoder () ,) . unwrap () ; let mut result = Vec :: new () ; Read :: read_to_end (& mut stream , & mut result) . unwrap () ; let original_f = BufReader :: new (fs :: File :: open ("./decodecorpus_files/z000068") . unwrap ()) ; let original : Vec < u8 > = original_f . bytes () . map (| x | x . unwrap ()) . collect () ; std :: println ! ("Results for file:") ; if original . len () != result . len () { panic ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { panic ! ("Result differs in at least {} bytes from original" , counter) ; } }
    };
}

test_streaming!()