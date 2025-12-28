macro_rules! Test {
    () => {
        # [derive (Serialize)] struct Test { a : String , b : String , c : Vec < String > , }
    };
}

Test!();