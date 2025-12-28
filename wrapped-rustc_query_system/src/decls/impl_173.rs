macro_rules! deps {
    () => {
        QueryJobInfo!();
        QueryMap!();
        CycleError!();
        QueryJobId!();
        QueryInfo!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl QueryJobId { pub (super) fn find_cycle_in_stack < I : Clone > (& self , query_map : QueryMap < I > , current_job : & Option < QueryJobId > , span : Span ,) -> CycleError < I > { let mut cycle = Vec :: new () ; let mut current_job = Option :: clone (current_job) ; while let Some (job) = current_job { let info = query_map . get (& job) . unwrap () ; cycle . push (QueryInfo { span : info . job . span , query : info . query . clone () }) ; if job == * self { cycle . reverse () ; cycle [0] . span = span ; let usage = info . job . parent . as_ref () . map (| parent | (info . job . span , parent . query (& query_map))) ; return CycleError { usage , cycle } ; } current_job = info . job . parent ; } panic ! ("did not find a cycle") } # [cold] # [inline (never)] pub fn find_dep_kind_root < I : Clone > (& self , query_map : QueryMap < I >) -> (QueryJobInfo < I > , usize) { let mut depth = 1 ; let info = query_map . get (& self) . unwrap () ; let dep_kind = info . query . dep_kind ; let mut current_id = info . job . parent ; let mut last_layout = (info . clone () , depth) ; while let Some (id) = current_id { let info = query_map . get (& id) . unwrap () ; if info . query . dep_kind == dep_kind { depth += 1 ; last_layout = (info . clone () , depth) ; } current_id = info . job . parent ; } last_layout } }
    };
}

impl_173!();