macro_rules! Binding {
    () => {
        # [derive (Clone , Copy , Debug)] struct Binding < 'tcx > { span : Span , source : Place < 'tcx > , var_id : LocalVarId , binding_mode : BindingMode , }
    };
}

Binding!()