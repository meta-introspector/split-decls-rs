macro_rules! deps {
    () => {
        FrameDecoder!();
        Read!();
    };
}

macro_rules! test_decode_from_to {
    () => {
        deps!();
        # [test] fn test_decode_from_to () { use crate :: decoding :: FrameDecoder ; use std :: fs :: File ; use std :: io :: BufReader ; use std :: io :: Read ; let f = BufReader :: new (File :: open ("./decodecorpus_files/z000088.zst") . unwrap ()) ; let mut frame_dec = FrameDecoder :: new () ; let content : Vec < u8 > = f . bytes () . map (| x | x . unwrap ()) . collect () ; let mut target = vec ! [0u8 ; 1024 * 1024] ; let source1 = & content [.. 50 * 1024] ; let (read1 , written1) = frame_dec . decode_from_to (source1 , target . as_mut_slice ()) . unwrap () ; let source2 = & content [read1 .. content . len () - 4] ; let (read2 , written2) = frame_dec . decode_from_to (source2 , & mut target [written1 ..]) . unwrap () ; assert ! (read1 + read2 == content . len () - 4) ; let chksum_source = & content [read1 + read2 ..] ; let (read3 , written3) = frame_dec . decode_from_to (chksum_source , & mut target [written1 + written2 ..]) . unwrap () ; assert ! (read3 == 4) ; assert ! (written3 == 0) ; let read = read1 + read2 + read3 ; let written = written1 + written2 ; let result = & target . as_slice () [.. written] ; if read != content . len () { panic ! ("Byte counter: {} was wrong. Should be: {}" , read , content . len ()) ; } match frame_dec . get_checksum_from_data () { Some (chksum) => { # [cfg (feature = "hash")] if frame_dec . get_calculated_checksum () . unwrap () != chksum { std :: println ! ("Checksum did not match! From data: {}, calculated while decoding: {}\n" , chksum , frame_dec . get_calculated_checksum () . unwrap ()) ; } else { std :: println ! ("Checksums are ok!\n") ; } # [cfg (not (feature = "hash"))] std :: println ! ("Checksum feature not enabled, skipping. From data: {}\n" , chksum) ; } None => std :: println ! ("No checksums to test\n") , } let original_f = BufReader :: new (File :: open ("./decodecorpus_files/z000088") . unwrap ()) ; let original : Vec < u8 > = original_f . bytes () . map (| x | x . unwrap ()) . collect () ; if original . len () != result . len () { panic ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { panic ! ("Result differs in at least {} bytes from original" , counter) ; } }
    };
}

test_decode_from_to!()