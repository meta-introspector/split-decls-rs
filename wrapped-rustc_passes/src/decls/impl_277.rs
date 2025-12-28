macro_rules! deps {
    () => {
        FeaturePreviouslyDeclared!();
        FeatureStableTwice!();
        LibFeatureCollector!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < 'tcx > LibFeatureCollector < 'tcx > { fn new (tcx : TyCtxt < 'tcx >) -> LibFeatureCollector < 'tcx > { LibFeatureCollector { tcx , lib_features : LibFeatures :: default () } } fn extract (& self , attr : & Attribute) -> Option < (Symbol , FeatureStability , Span) > { let (feature , level , span) = match attr { Attribute :: Parsed (AttributeKind :: Stability { stability , span }) => { (stability . feature , stability . level , * span) } Attribute :: Parsed (AttributeKind :: ConstStability { stability , span }) => { (stability . feature , stability . level , * span) } Attribute :: Parsed (AttributeKind :: BodyStability { stability , span }) => { (stability . feature , stability . level , * span) } _ => return None , } ; let feature_stability = match level { StabilityLevel :: Unstable { old_name , .. } => FeatureStability :: Unstable { old_name } , StabilityLevel :: Stable { since , .. } => FeatureStability :: AcceptedSince (match since { StableSince :: Version (v) => Symbol :: intern (& v . to_string ()) , StableSince :: Current => sym :: env_CFG_RELEASE , StableSince :: Err (_) => return None , }) , } ; Some ((feature , feature_stability , span)) } fn collect_feature (& mut self , feature : Symbol , stability : FeatureStability , span : Span) { let existing_stability = self . lib_features . stability . get (& feature) . cloned () ; match (stability , existing_stability) { (_ , None) => { self . lib_features . stability . insert (feature , (stability , span)) ; } (FeatureStability :: AcceptedSince (since) , Some ((FeatureStability :: AcceptedSince (prev_since) , _)) ,) => { if prev_since != since { self . tcx . dcx () . emit_err (FeatureStableTwice { span , feature , since , prev_since , }) ; } } (FeatureStability :: AcceptedSince (_) , Some ((FeatureStability :: Unstable { .. } , _))) => { self . tcx . dcx () . emit_err (FeaturePreviouslyDeclared { span , feature , declared : "stable" , prev_declared : "unstable" , }) ; } (FeatureStability :: Unstable { .. } , Some ((FeatureStability :: AcceptedSince (_) , _))) => { self . tcx . dcx () . emit_err (FeaturePreviouslyDeclared { span , feature , declared : "unstable" , prev_declared : "stable" , }) ; } (FeatureStability :: Unstable { .. } , Some ((FeatureStability :: Unstable { .. } , _))) => { } } } }
    };
}

impl_277!();