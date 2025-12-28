macro_rules! Name {
    () => {
        # [derive (Clone)] pub struct Name { pub value : String , pub span : Span , }
    };
}

Name!();