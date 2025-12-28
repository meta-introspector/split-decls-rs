macro_rules! mark_code_coverage_dead_code_cgu {
    () => {
        fn mark_code_coverage_dead_code_cgu < 'tcx > (codegen_units : & mut [CodegenUnit < 'tcx >]) { assert ! (! codegen_units . is_empty ()) ; let dead_code_cgu = codegen_units . iter_mut () . filter (| cgu | cgu . items () . iter () . any (| (_ , data) | data . linkage == Linkage :: External)) . min_by_key (| cgu | cgu . size_estimate ()) ; let dead_code_cgu = if let Some (cgu) = dead_code_cgu { cgu } else { & mut codegen_units [0] } ; dead_code_cgu . make_code_coverage_dead_code_cgu () ; }
    };
}

mark_code_coverage_dead_code_cgu!()