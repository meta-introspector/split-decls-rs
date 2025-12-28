macro_rules! assert_eq_streams {
    () => {
        # [track_caller] fn assert_eq_streams (expect : TokenStream , res : TokenStream) { let pretty = | ts : TokenStream | prettyplease :: unparse (& syn :: parse_file (& ts . to_string ()) . unwrap ()) ; let expect = pretty (expect . clone ()) ; let res = pretty (res . clone ()) ; if expect != res { let diff = dissimilar :: diff (& expect , & res) . into_iter () . flat_map (| chunk | { let (prefix , chunk) = match chunk { Chunk :: Equal (chunk) => (" " , chunk) , Chunk :: Delete (chunk) => ("-" , chunk) , Chunk :: Insert (chunk) => ("+" , chunk) , } ; [prefix , chunk , "\n"] }) . collect :: < String > () ; panic ! ("\
test failed:
got:
```
{}
```

diff (expected vs got):
```
{}
```\n" , res , diff) ; } }
    };
}

assert_eq_streams!()