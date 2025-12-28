macro_rules! deps {
    () => {
        StreamingDecoder!();
        Read!();
    };
}

macro_rules! test_streaming_no_std {
    () => {
        deps!();
        # [test] # [cfg (not (feature = "std"))] fn test_streaming_no_std () { use crate :: io :: Read ; let content = include_bytes ! ("../../decodecorpus_files/z000088.zst") ; let mut content = content . as_slice () ; let mut stream = crate :: decoding :: StreamingDecoder :: new (& mut content) . unwrap () ; let original = include_bytes ! ("../../decodecorpus_files/z000088") ; let mut result = vec ! [0 ; original . len ()] ; Read :: read_exact (& mut stream , & mut result) . unwrap () ; if original . len () != result . len () { panic ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { panic ! ("Result differs in at least {} bytes from original" , counter) ; } let content = include_bytes ! ("../../decodecorpus_files/z000068.zst") ; let mut content = content . as_slice () ; let mut stream = crate :: decoding :: StreamingDecoder :: new_with_decoder (& mut content , stream . into_frame_decoder () ,) . unwrap () ; let original = include_bytes ! ("../../decodecorpus_files/z000068") ; let mut result = vec ! [0 ; original . len ()] ; Read :: read_exact (& mut stream , & mut result) . unwrap () ; std :: println ! ("Results for file:") ; if original . len () != result . len () { panic ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { panic ! ("Result differs in at least {} bytes from original" , counter) ; } }
    };
}

test_streaming_no_std!()