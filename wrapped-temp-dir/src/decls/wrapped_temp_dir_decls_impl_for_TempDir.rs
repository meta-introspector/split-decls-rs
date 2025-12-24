use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl TempDir {
    fn remove_dir(path: &Path) -> Result<(), std::io::Error> {
        match std::fs::remove_dir_all(path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
            Err(e) => {
                Err(
                    std::io::Error::new(
                        e.kind(),
                        format!("error removing directory and contents {path:?}: {e}"),
                    ),
                )
            }
        }
    }
    /// Create a new empty directory in a system temporary directory.
    ///
    /// Drop the struct to delete the directory and everything under it.
    /// Deletes symbolic links and does not follow them.
    ///
    /// Ignores any error while deleting.
    /// See [`TempDir::panic_on_cleanup_error`](struct.TempDir.html#method.panic_on_cleanup_error).
    ///
    /// # Errors
    /// Returns `Err` when it fails to create the directory.
    ///
    /// # Example
    /// ```rust
    /// // Prints "/tmp/t1a9b-0".
    /// println!("{:?}", temp_dir::TempDir::new().unwrap().path());
    /// ```
    pub fn new() -> Result<Self, std::io::Error> {
        Self::with_prefix("t")
    }
    /// Create a new empty directory in a system temporary directory.
    /// Use `prefix` as the first part of the directory's name.
    ///
    /// Drop the struct to delete the directory and everything under it.
    /// Deletes symbolic links and does not follow them.
    ///
    /// Ignores any error while deleting.
    /// See [`TempDir::panic_on_cleanup_error`](struct.TempDir.html#method.panic_on_cleanup_error).
    ///
    /// # Errors
    /// Returns `Err` when it fails to create the directory.
    ///
    /// # Example
    /// ```rust
    /// // Prints "/tmp/ok1a9b-0".
    /// println!("{:?}", temp_dir::TempDir::with_prefix("ok").unwrap().path());
    /// ```
    pub fn with_prefix(prefix: impl AsRef<str>) -> Result<Self, std::io::Error> {
        loop {
            let path_buf = std::env::temp_dir()
                .join(
                    format!(
                        "{}{:x}-{:x}", prefix.as_ref(), std::process::id(),
                        INTERNAL_COUNTER.fetch_add(1, Ordering::AcqRel),
                    ),
                );
            match std::fs::create_dir(&path_buf) {
                Err(
                    e,
                ) if e.kind() == ErrorKind::AlreadyExists
                    && INTERNAL_RETRY.load(Ordering::Acquire) => {}
                Err(e) => {
                    return Err(
                        std::io::Error::new(
                            e.kind(),
                            format!("error creating directory {path_buf:?}: {e}"),
                        ),
                    );
                }
                Ok(()) => {
                    return Ok(Self {
                        delete_on_drop: true,
                        panic_on_delete_err: false,
                        path_buf,
                    });
                }
            }
        }
    }
    /// Remove the directory and its contents now.
    ///
    /// # Errors
    /// Returns an error if the directory exists and we fail to remove it and its contents.
    #[allow(clippy::missing_panics_doc)]
    pub fn cleanup(self) -> Result<(), std::io::Error> {
        Self::remove_dir(&self.path_buf)
    }
    /// Make the struct panic on drop if it hits an error while
    /// removing the directory or its contents.
    #[must_use]
    pub fn panic_on_cleanup_error(mut self) -> Self {
        self.panic_on_delete_err = true;
        self
    }
    /// Do not delete the directory or its contents.
    ///
    /// This is useful when debugging a test.
    pub fn leak(mut self) {
        self.delete_on_drop = false;
    }
    /// Do not delete the directory or its contents on Drop.
    #[must_use]
    pub fn dont_delete_on_drop(mut self) -> Self {
        self.delete_on_drop = false;
        self
    }
    /// The path to the directory.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn path(&self) -> &Path {
        &self.path_buf
    }
    /// The path to `name` under the directory.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn child(&self, name: impl AsRef<str>) -> PathBuf {
        let mut result = self.path_buf.clone();
        result.push(name.as_ref());
        result
    }
}
