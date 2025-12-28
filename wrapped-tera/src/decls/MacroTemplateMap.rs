macro_rules! deps {
    () => {
        MacroNamespaceMap!();
    };
}

macro_rules! MacroTemplateMap {
    () => {
        deps!();
        # [doc = " Maps { template => { namespace => ( macro_template, { macro => macro_definition }) }"] pub type MacroTemplateMap < 'a > = HashMap < & 'a str , MacroNamespaceMap < 'a > > ;
    };
}

MacroTemplateMap!();