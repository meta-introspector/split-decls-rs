macro_rules! deps {
    () => {
        ZipResult!();
    };
}

macro_rules! test_bad_extended_timestamp {
    () => {
        deps!();
        # [test] # [doc = " Ensure we don't panic or read garbage data if the field body is empty"] pub fn test_bad_extended_timestamp () -> ZipResult < () > { use crate :: ZipArchive ; use std :: io :: Cursor ; assert ! (ZipArchive :: new (Cursor :: new (include_bytes ! ("../../tests/data/extended_timestamp_bad.zip"))) . is_err ()) ; Ok (()) }
    };
}

test_bad_extended_timestamp!();