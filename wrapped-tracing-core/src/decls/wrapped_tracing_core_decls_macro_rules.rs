use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Statically constructs new span [metadata].
///
/// /// For example:
/// ```rust
/// # use tracing_core::{callsite::Callsite, subscriber::Interest};
/// use tracing_core::metadata;
/// use tracing_core::metadata::{Kind, Level, Metadata};
/// # fn main() {
/// # pub struct MyCallsite { }
/// # impl Callsite for MyCallsite {
/// # fn set_interest(&self, _: Interest) { unimplemented!() }
/// # fn metadata(&self) -> &Metadata { unimplemented!() }
/// # }
/// #
/// static FOO_CALLSITE: MyCallsite = MyCallsite {
///     // ...
/// };
///
/// static FOO_METADATA: Metadata = metadata!{
///     name: "foo",
///     target: module_path!(),
///     level: Level::DEBUG,
///     fields: &["bar", "baz"],
///     callsite: &FOO_CALLSITE,
///     kind: Kind::SPAN,
/// };
/// # }
/// ```
///
/// [metadata]: metadata::Metadata
/// [`Metadata::new`]: metadata::Metadata::new
#[macro_export]
macro_rules! metadata {
    (
        name : $name:expr, target : $target:expr, level : $level:expr, fields :
        $fields:expr, callsite : $callsite:expr, kind : $kind:expr
    ) => {
        $crate::metadata! { name : $name, target : $target, level : $level, fields :
        $fields, callsite : $callsite, kind : $kind, }
    };
    (
        name : $name:expr, target : $target:expr, level : $level:expr, fields :
        $fields:expr, callsite : $callsite:expr, kind : $kind:expr,
    ) => {
        $crate::metadata::Metadata::new($name, $target, $level,
        $crate::__macro_support::Option::Some($crate::__macro_support::file!()),
        $crate::__macro_support::Option::Some($crate::__macro_support::line!()),
        $crate::__macro_support::Option::Some($crate::__macro_support::module_path!()),
        $crate::field::FieldSet::new($fields, $crate::identify_callsite!($callsite)),
        $kind,)
    };
}
