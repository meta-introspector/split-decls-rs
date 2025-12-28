macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! git_version_from_bytes {
    () => {
        deps!();
        fn git_version_from_bytes (bytes : & [u8]) -> Result < (u8 , u8 , u8) > { let mut numbers = bytes . split (| b | * b == b' ' || * b == b'\n') . nth (2) . expect ("git version <version>") . split (| b | * b == b'.') . take (3) . map (| n | std :: str :: from_utf8 (n) . expect ("valid utf8 in version number")) . map (u8 :: from_str) ; Ok ((| | -> Result < _ > { Ok ((numbers . next () . expect ("major") ? , numbers . next () . expect ("minor") ? , numbers . next () . expect ("patch") ? ,)) }) () . map_err (| err | { format ! ("Could not parse version from output of 'git --version' ({:?}) with error: {}" , bytes . to_str_lossy () , err) }) ?) }
    };
}

git_version_from_bytes!();