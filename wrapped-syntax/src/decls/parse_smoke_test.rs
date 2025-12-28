macro_rules! parse_smoke_test {
    () => {
        # [test] fn parse_smoke_test () { let code = r#"
fn main() {
    println!("Hello, world!")
}
    "# ; let parse = SourceFile :: parse (code , Edition :: CURRENT) ; assert ! (parse . ok () . is_ok ()) ; }
    };
}

parse_smoke_test!()