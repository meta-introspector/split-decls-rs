use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[test]
fn deserialize_newtype2() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, Deserialize, PartialEq)]
    struct Bar {
        foo: Foo,
    }
    let input = Value::Map(
        vec![(Value::String("foo".to_owned()), Value::I32(5))]
            .into_iter()
            .collect(),
    );
    let bar = Bar::deserialize(input).unwrap();
    assert_eq!(bar, Bar { foo: Foo(5) });
}
