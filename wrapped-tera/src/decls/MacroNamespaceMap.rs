macro_rules! deps {
    () => {
        MacroDefinitionMap!();
    };
}

macro_rules! MacroNamespaceMap {
    () => {
        deps!();
        # [doc = " Maps { namespace => ( macro_template, { macro => macro_definition }) }"] pub type MacroNamespaceMap < 'a > = HashMap < & 'a str , (& 'a str , & 'a MacroDefinitionMap) > ;
    };
}

MacroNamespaceMap!()