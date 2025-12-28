macro_rules! deps {
    () => {
        CanonicalPath!();
        Stderr!();
    };
}

macro_rules! ParsedOutputs {
    () => {
        deps!();
        struct ParsedOutputs { stdout : String , stderrs : Map < CanonicalPath , Stderr > , }
    };
}

ParsedOutputs!()