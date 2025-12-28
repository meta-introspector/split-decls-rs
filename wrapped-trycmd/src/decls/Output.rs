macro_rules! deps {
    () => {
        Filesystem!();
        Stream!();
        Spawn!();
    };
}

macro_rules! Output {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) struct Output { path : std :: path :: PathBuf , id : Option < String > , spawn : Spawn , stdout : Option < Stream > , stderr : Option < Stream > , fs : Filesystem , duration : Option < std :: time :: Duration > , }
    };
}

Output!()