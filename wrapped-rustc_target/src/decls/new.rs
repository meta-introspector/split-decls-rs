macro_rules! deps {
    () => {
        CrtObjects!();
    };
}

macro_rules! new {
    () => {
        deps!();
        pub (super) fn new (obj_table : & [(LinkOutputKind , & [& 'static str])]) -> CrtObjects { obj_table . iter () . map (| (z , k) | (* z , k . iter () . map (| b | (* b) . into ()) . collect ())) . collect () }
    };
}

new!()