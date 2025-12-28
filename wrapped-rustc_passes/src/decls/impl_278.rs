macro_rules! deps {
    () => {
        LibFeatureCollector!();
    };
}

macro_rules! impl_278 {
    () => {
        deps!();
        impl < 'tcx > Visitor < 'tcx > for LibFeatureCollector < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_attribute (& mut self , attr : & 'tcx Attribute) { if let Some ((feature , stable , span)) = self . extract (attr) { self . collect_feature (feature , stable , span) ; } } }
    };
}

impl_278!()