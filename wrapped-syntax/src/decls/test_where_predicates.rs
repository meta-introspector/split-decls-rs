macro_rules! test_where_predicates {
    () => {
        # [test] fn test_where_predicates () { fn assert_bound (text : & str , bound : Option < TypeBound >) { assert_eq ! (text , bound . unwrap () . syntax () . text () . to_string ()) ; } let file = SourceFile :: parse (r#"
fn foo()
where
   T: Clone + Copy + Debug + 'static,
   'a: 'b + 'c,
   Iterator::Item: 'a + Debug,
   Iterator::Item: Debug + 'a,
   <T as Iterator>::Item: Debug + 'a,
   for<'a> F: Fn(&'a str)
{}
        "# , parser :: Edition :: CURRENT ,) . ok () . unwrap () ; let where_clause = file . syntax () . descendants () . find_map (WhereClause :: cast) . unwrap () ; let mut predicates = where_clause . predicates () ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert ! (pred . for_binder () . is_none ()) ; assert_eq ! ("T" , pred . ty () . unwrap () . syntax () . text () . to_string ()) ; assert_bound ("Clone" , bounds . next ()) ; assert_bound ("Copy" , bounds . next ()) ; assert_bound ("Debug" , bounds . next ()) ; assert_bound ("'static" , bounds . next ()) ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert_eq ! ("'a" , pred . lifetime () . unwrap () . lifetime_ident_token () . unwrap () . text ()) ; assert_bound ("'b" , bounds . next ()) ; assert_bound ("'c" , bounds . next ()) ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert_eq ! ("Iterator::Item" , pred . ty () . unwrap () . syntax () . text () . to_string ()) ; assert_bound ("'a" , bounds . next ()) ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert_eq ! ("Iterator::Item" , pred . ty () . unwrap () . syntax () . text () . to_string ()) ; assert_bound ("Debug" , bounds . next ()) ; assert_bound ("'a" , bounds . next ()) ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert_eq ! ("<T as Iterator>::Item" , pred . ty () . unwrap () . syntax () . text () . to_string ()) ; assert_bound ("Debug" , bounds . next ()) ; assert_bound ("'a" , bounds . next ()) ; let pred = predicates . next () . unwrap () ; let mut bounds = pred . type_bound_list () . unwrap () . bounds () ; assert_eq ! ("<'a>" , pred . for_binder () . unwrap () . generic_param_list () . unwrap () . syntax () . text () . to_string ()) ; assert_eq ! ("F" , pred . ty () . unwrap () . syntax () . text () . to_string ()) ; assert_bound ("Fn(&'a str)" , bounds . next ()) ; }
    };
}

test_where_predicates!();