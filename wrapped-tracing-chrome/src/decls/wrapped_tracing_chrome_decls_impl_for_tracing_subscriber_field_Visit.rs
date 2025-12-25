use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a> tracing_subscriber::field::Visit for JsonVisitor<'a> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.object
            .insert(field.name().to_owned(), format!("{value:?}").into());
    }
}
