macro_rules! deps {
    () => {
        Document!();
    };
}

macro_rules! ImDocument {
    () => {
        deps!();
        # [doc = " Type representing a parsed TOML document"] # [deprecated (since = "0.23.0" , note = "Replaced with `Document`")] pub type ImDocument < S > = Document < S > ;
    };
}

ImDocument!()