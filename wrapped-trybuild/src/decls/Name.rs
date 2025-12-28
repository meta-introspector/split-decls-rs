macro_rules! Name {
    () => {
        # [derive (Serialize , Clone , Debug)] pub (crate) struct Name (pub String) ;
    };
}

Name!()