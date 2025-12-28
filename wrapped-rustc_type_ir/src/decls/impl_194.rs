macro_rules! deps {
    () => {
        Interner!();
        ExternalConstraintsData!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < I : Interner > ExternalConstraintsData < I > { pub fn is_empty (& self) -> bool { self . region_constraints . is_empty () && self . opaque_types . is_empty () && self . normalization_nested_goals . is_empty () } }
    };
}

impl_194!()