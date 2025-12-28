macro_rules! deps {
    () => {
        Query!();
    };
}

macro_rules! add_query_desc_cached_impl {
    () => {
        deps!();
        # [doc = " Add the impl of QueryDescription for the query to `impls` if one is requested"] fn add_query_desc_cached_impl (query : & Query , descs : & mut proc_macro2 :: TokenStream , cached : & mut proc_macro2 :: TokenStream ,) { let Query { name , key , modifiers , .. } = & query ; let ra_hint = quote ! { let crate :: query :: Providers { # name : _ , .. } ; } ; let cache = if let Some ((args , expr)) = modifiers . cache . as_ref () { let tcx = args . as_ref () . map (| t | quote ! { # t }) . unwrap_or_else (| | quote ! { _ }) ; quote ! { # [allow (unused_variables , unused_braces , rustc :: pass_by_value)] # [inline] pub fn # name <'tcx > (# tcx : TyCtxt <'tcx >, # key : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint # expr } } } else { quote ! { # [allow (rustc :: pass_by_value)] # [inline] pub fn # name <'tcx > (_ : TyCtxt <'tcx >, _ : & crate :: query :: queries ::# name :: Key <'tcx >) -> bool { # ra_hint false } } } ; let (tcx , desc) = & modifiers . desc ; let tcx = tcx . as_ref () . map_or_else (| | quote ! { _ } , | t | quote ! { # t }) ; let desc = quote ! { # [allow (unused_variables)] pub fn # name <'tcx > (tcx : TyCtxt <'tcx >, key : crate :: query :: queries ::# name :: Key <'tcx >) -> String { let (# tcx , # key) = (tcx , key) ; :: rustc_middle :: ty :: print :: with_no_trimmed_paths ! (format ! (# desc)) } } ; descs . extend (quote ! { # desc }) ; cached . extend (quote ! { # cache }) ; }
    };
}

add_query_desc_cached_impl!()