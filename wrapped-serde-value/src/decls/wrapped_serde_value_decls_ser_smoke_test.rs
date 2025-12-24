use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[test]
fn ser_smoke_test() {
    #[derive(Serialize)]
    struct Foo {
        a: u32,
        b: String,
        c: Vec<bool>,
    }
    let foo = Foo {
        a: 15,
        b: "hello".into(),
        c: vec![true, false],
    };
    let expected = Value::Map(
        vec![
            (Value::String("a".into()), Value::U32(15)), (Value::String("b".into()),
            Value::String("hello".into())), (Value::String("c".into()),
            Value::Seq(vec![Value::Bool(true), Value::Bool(false)]),),
        ]
            .into_iter()
            .collect(),
    );
    let value = to_value(&foo).unwrap();
    assert_eq!(expected, value);
}
