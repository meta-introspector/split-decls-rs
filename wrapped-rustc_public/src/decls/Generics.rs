macro_rules! deps {
    () => {
        GenericParamDef!();
        Span!();
    };
}

macro_rules! Generics {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Generics { pub parent : Option < GenericDef > , pub parent_count : usize , pub params : Vec < GenericParamDef > , pub param_def_id_to_index : Vec < (GenericDef , u32) > , pub has_self : bool , pub has_late_bound_regions : Option < Span > , }
    };
}

Generics!();