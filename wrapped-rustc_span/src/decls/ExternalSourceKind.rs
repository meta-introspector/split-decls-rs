macro_rules! deps {
    () => {
        SourceFile!();
    };
}

macro_rules! ExternalSourceKind {
    () => {
        deps!();
        # [doc = " The state of the lazy external source loading mechanism of a `SourceFile`."] # [derive (PartialEq , Eq , Clone , Debug)] pub enum ExternalSourceKind { # [doc = " The external source has been loaded already."] Present (Arc < String >) , # [doc = " No attempt has been made to load the external source."] AbsentOk , # [doc = " A failed attempt has been made to load the external source."] AbsentErr , }
    };
}

ExternalSourceKind!()