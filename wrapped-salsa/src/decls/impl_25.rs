macro_rules! deps {
    () => {
        Backtrace!();
        CapturedQuery!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Display for Backtrace { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { writeln ! (fmt , "query stacktrace:") ? ; let full = fmt . alternate () ; let indent = "             " ; for (idx , & CapturedQuery { database_key_index , durability , changed_at , ref cycle_heads , iteration_count , } ,) in self . 0 . iter () . enumerate () { write ! (fmt , "{idx:>4}: {database_key_index:?}") ? ; if full { write ! (fmt , " -> ({changed_at:?}, {durability:#?}") ? ; if ! cycle_heads . is_empty () || ! iteration_count . is_initial () { write ! (fmt , ", iteration = {iteration_count}") ? ; } write ! (fmt , ")") ? ; } writeln ! (fmt) ? ; crate :: attach :: with_attached_database (| db | { let ingredient = db . zalsa () . lookup_ingredient (database_key_index . ingredient_index ()) ; let loc = ingredient . location () ; writeln ! (fmt , "{indent}at {}:{}" , loc . file , loc . line) ? ; if ! cycle_heads . is_empty () { write ! (fmt , "{indent}cycle heads: ") ? ; for (idx , head) in cycle_heads . iter () . enumerate () { if idx != 0 { write ! (fmt , ", ") ? ; } write ! (fmt , "{:?} -> iteration = {}" , head . database_key_index , head . iteration_count) ? ; } writeln ! (fmt) ? ; } Ok (()) }) . transpose () ? ; } Ok (()) } }
    };
}

impl_25!();