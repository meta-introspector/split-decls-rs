macro_rules! IncrementCompilation {
    () => {
        # [derive (Diagnostic)] # [diag (query_system_increment_compilation)] # [help] # [note (query_system_increment_compilation_note1)] # [note (query_system_increment_compilation_note2)] pub (crate) struct IncrementCompilation { pub run_cmd : String , pub dep_node : String , }
    };
}

IncrementCompilation!();