macro_rules! deps {
    () => {
        AdjacentEdges!();
        NodeIndex!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'g , N : Debug , E : Debug > AdjacentEdges < 'g , N , E > { fn targets (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . target) } fn sources (self) -> impl Iterator < Item = NodeIndex > { self . map (| (_ , edge) | edge . source) } }
    };
}

impl_133!();