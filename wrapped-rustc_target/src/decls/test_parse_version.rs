macro_rules! deps {
    () => {
        OSVersion!();
    };
}

macro_rules! test_parse_version {
    () => {
        deps!();
        # [test] fn test_parse_version () { assert_eq ! ("10" . parse () , Ok (OSVersion :: new (10 , 0 , 0))) ; assert_eq ! ("10.12" . parse () , Ok (OSVersion :: new (10 , 12 , 0))) ; assert_eq ! ("10.12.6" . parse () , Ok (OSVersion :: new (10 , 12 , 6))) ; assert_eq ! ("9999.99.99" . parse () , Ok (OSVersion :: new (9999 , 99 , 99))) ; }
    };
}

test_parse_version!()