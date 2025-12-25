use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Parse<SourceFile> {
    pub fn debug_dump(&self) -> String {
        let mut buf = format!("{:#?}", self.tree().syntax());
        for err in self.errors() {
            format_to!(buf, "error {:?}: {}\n", err.range(), err);
        }
        buf
    }
    pub fn reparse(&self, delete: TextRange, insert: &str, edition: Edition) -> Parse<SourceFile> {
        self.incremental_reparse(delete, insert, edition)
            .unwrap_or_else(|| self.full_reparse(delete, insert, edition))
    }
    fn incremental_reparse(
        &self,
        delete: TextRange,
        insert: &str,
        edition: Edition,
    ) -> Option<Parse<SourceFile>> {
        parsing::incremental_reparse(
            self.tree().syntax(),
            delete,
            insert,
            self.errors.as_deref().unwrap_or_default().iter().cloned(),
            edition,
        )
        .map(|(green_node, errors, _reparsed_range)| Parse {
            green: Some(green_node),
            errors: if errors.is_empty() {
                None
            } else {
                Some(errors.into())
            },
            _ty: PhantomData,
        })
    }
    fn full_reparse(&self, delete: TextRange, insert: &str, edition: Edition) -> Parse<SourceFile> {
        let mut text = self.tree().syntax().text().to_string();
        text.replace_range(Range::<usize>::from(delete), insert);
        SourceFile::parse(&text, edition)
    }
}
