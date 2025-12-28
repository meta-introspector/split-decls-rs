macro_rules! deps {
    () => {
        MacroTemplateMap!();
    };
}

macro_rules! MacroCollection {
    () => {
        deps!();
        # [doc = " Collection of all macro templates by file"] # [derive (Clone , Debug , Default)] pub struct MacroCollection < 'a > { macros : MacroTemplateMap < 'a > , }
    };
}

MacroCollection!()