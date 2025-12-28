macro_rules! deps {
    () => {
        Error!();
        ExecuteSequencesError!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for ExecuteSequencesError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { ExecuteSequencesError :: DecodebufferError (source) => Some (source) , _ => None , } } }
    };
}

impl_84!();