use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'a, 'b> Builder<'a, 'b> {
    /// Create a new `Builder`.
    ///
    /// # Examples
    ///
    /// Create a named temporary file and write some data into it:
    ///
    /// ```
    /// use std::ffi::OsStr;
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .prefix("my-temporary-note")
    ///     .suffix(".txt")
    ///     .rand_bytes(5)
    ///     .tempfile()?;
    ///
    /// let name = named_tempfile
    ///     .path()
    ///     .file_name().and_then(OsStr::to_str);
    ///
    /// if let Some(name) = name {
    ///     assert!(name.starts_with("my-temporary-note"));
    ///     assert!(name.ends_with(".txt"));
    ///     assert_eq!(name.len(), "my-temporary-note.txt".len() + 5);
    /// }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// Create a temporary directory and add a file to it:
    ///
    /// ```
    /// use std::io::Write;
    /// use std::fs::File;
    /// use std::ffi::OsStr;
    /// use tempfile::Builder;
    ///
    /// let dir = Builder::new()
    ///     .prefix("my-temporary-dir")
    ///     .rand_bytes(5)
    ///     .tempdir()?;
    ///
    /// let file_path = dir.path().join("my-temporary-note.txt");
    /// let mut file = File::create(file_path)?;
    /// writeln!(file, "Brian was here. Briefly.")?;
    ///
    /// // By closing the `TempDir` explicitly, we can check that it has
    /// // been deleted successfully. If we don't close it explicitly,
    /// // the directory will still be deleted when `dir` goes out
    /// // of scope, but we won't know whether deleting the directory
    /// // succeeded.
    /// drop(file);
    /// dir.close()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// Create a temporary directory with a chosen prefix under a chosen folder:
    ///
    /// ```no_run
    /// use tempfile::Builder;
    ///
    /// let dir = Builder::new()
    ///     .prefix("my-temporary-dir")
    ///     .tempdir_in("folder-with-tempdirs")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set a custom filename prefix.
    ///
    /// Path separators are legal but not advisable.
    /// Default: `.tmp`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .prefix("my-temporary-note")
    ///     .tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn prefix<S: AsRef<OsStr> + ?Sized>(&mut self, prefix: &'a S) -> &mut Self {
        self.prefix = prefix.as_ref();
        self
    }
    /// Set a custom filename suffix.
    ///
    /// Path separators are legal but not advisable.
    /// Default: empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .suffix(".txt")
    ///     .tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn suffix<S: AsRef<OsStr> + ?Sized>(&mut self, suffix: &'b S) -> &mut Self {
        self.suffix = suffix.as_ref();
        self
    }
    /// Set the number of random bytes.
    ///
    /// Default: `6`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .rand_bytes(5)
    ///     .tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn rand_bytes(&mut self, rand: usize) -> &mut Self {
        self.random_len = rand;
        self
    }
    /// Configure the file to be opened in append-only mode.
    ///
    /// Default: `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .append(true)
    ///     .tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn append(&mut self, append: bool) -> &mut Self {
        self.append = append;
        self
    }
    /// Set the permissions for the new temporary file/directory.
    ///
    /// # Platform Notes
    ///
    /// ## Windows
    ///
    /// This setting is only fully-supported on unix-like platforms. On Windows, if this method is
    /// called with a [`Permissions`] object where `permissions.readonly` returns true, creating
    /// temporary files and directories will fail with an error.
    ///
    /// ## Unix
    ///
    /// On unix-like systems, the actual permission bits set on the tempfile or tempdir will be
    /// affected by the `umask` applied by the underlying syscall. The actual permission bits are
    /// calculated via `permissions & !umask`. In other words, depending on your umask, the
    /// permissions of the created file may be more restrictive (but never more permissive) than the
    /// ones you specified.
    ///
    /// Permissions default to `0o600` for tempfiles and `0o777` for tempdirs. Note, this doesn't
    /// include effects of the current `umask`. For example, combined with the standard umask
    /// `0o022`, the defaults yield `0o600` for tempfiles and `0o755` for tempdirs.
    ///
    /// ## WASI
    ///
    /// While custom permissions are allowed on WASI, they will be ignored as the platform has no
    /// concept of permissions or file modes (or multiple users for that matter).
    ///
    /// # Examples
    ///
    /// Create a named temporary file that is world-readable.
    ///
    /// ```
    /// # #[cfg(unix)]
    /// # {
    /// use tempfile::Builder;
    /// use std::os::unix::fs::PermissionsExt;
    ///
    /// let all_read_write = std::fs::Permissions::from_mode(0o666);
    /// let tempfile = Builder::new().permissions(all_read_write).tempfile()?;
    ///
    /// // Check that this worked and that the file is world-readable.
    /// //
    /// // NOTE: the file likely won't actually be created with 0o666 permissions because it's
    /// // restricted by the user's umask.
    /// //
    /// // NOTE: This test will fail if the user's umask is, e.g., 0o066.
    /// let actual_permissions = tempfile.path().metadata()?.permissions();
    /// assert_eq!(actual_permissions.mode() & 0o044, 0o044);
    /// # }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// Create a named temporary directory that is restricted to the owner.
    ///
    /// ```
    /// # #[cfg(unix)]
    /// # {
    /// use tempfile::Builder;
    /// use std::os::unix::fs::PermissionsExt;
    ///
    /// let owner_rwx = std::fs::Permissions::from_mode(0o700);
    /// let tempdir = Builder::new().permissions(owner_rwx).tempdir()?;
    /// let actual_permissions = tempdir.path().metadata()?.permissions();
    /// assert_eq!(
    ///     actual_permissions.mode() & !0o170000,
    ///     0o700,
    ///     "we get the narrow permissions we asked for"
    /// );
    /// # }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn permissions(&mut self, permissions: Permissions) -> &mut Self {
        self.permissions = Some(permissions);
        self
    }
    /// Disable cleanup of the file/folder to even when the [`NamedTempFile`]/[`TempDir`] goes out
    /// of scope. Prefer [`NamedTempFile::keep`] and [`TempDir::keep`] where possible;
    /// `disable_cleanup` is provided for testing & debugging.
    ///
    /// By default, the file/folder is automatically cleaned up in the destructor of
    /// [`NamedTempFile`]/[`TempDir`]. When `disable_cleanup` is set to `true`, this behavior is
    /// suppressed. If you wish to disable cleanup after creating a temporary file/directory, call
    /// [`NamedTempFile::disable_cleanup`] or [`TempDir::disable_cleanup`].
    ///
    /// # Warnings
    ///
    /// On some platforms (for now, only Windows), temporary files are marked with a special
    /// "temporary file" (`FILE_ATTRIBUTE_TEMPORARY`) attribute. Disabling cleanup _will not_ unset
    /// this attribute while calling [`NamedTempFile::keep`] will.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let named_tempfile = Builder::new()
    ///     .disable_cleanup(true)
    ///     .tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn disable_cleanup(&mut self, disable_cleanup: bool) -> &mut Self {
        self.disable_cleanup = disable_cleanup;
        self
    }
    /// Deprecated alias for [`Builder::disable_cleanup`].
    #[deprecated = "Use Builder::disable_cleanup"]
    pub fn keep(&mut self, keep: bool) -> &mut Self {
        self.disable_cleanup(keep)
    }
    /// Create the named temporary file.
    ///
    /// # Security
    ///
    /// See [the security][security] docs on `NamedTempFile`.
    ///
    /// # Resource leaking
    ///
    /// See [the resource leaking][resource-leaking] docs on `NamedTempFile`.
    ///
    /// # Errors
    ///
    /// If the file cannot be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let tempfile = Builder::new().tempfile()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// [security]: struct.NamedTempFile.html#security
    /// [resource-leaking]: struct.NamedTempFile.html#resource-leaking
    pub fn tempfile(&self) -> io::Result<NamedTempFile> {
        self.tempfile_in(env::temp_dir())
    }
    /// Create the named temporary file in the specified directory.
    ///
    /// # Security
    ///
    /// See [the security][security] docs on `NamedTempFile`.
    ///
    /// # Resource leaking
    ///
    /// See [the resource leaking][resource-leaking] docs on `NamedTempFile`.
    ///
    /// # Errors
    ///
    /// If the file cannot be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let tempfile = Builder::new().tempfile_in("./")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// [security]: struct.NamedTempFile.html#security
    /// [resource-leaking]: struct.NamedTempFile.html#resource-leaking
    pub fn tempfile_in<P: AsRef<Path>>(&self, dir: P) -> io::Result<NamedTempFile> {
        util::create_helper(
            dir.as_ref(),
            self.prefix,
            self.suffix,
            self.random_len,
            |path| {
                file::create_named(
                    path,
                    OpenOptions::new().append(self.append),
                    self.permissions.as_ref(),
                    self.disable_cleanup,
                )
            },
        )
    }
    /// Attempts to make a temporary directory inside of [`env::temp_dir()`] whose
    /// name will have the prefix, `prefix`. The directory and
    /// everything inside it will be automatically deleted once the
    /// returned `TempDir` is destroyed.
    ///
    /// # Resource leaking
    ///
    /// See [the resource leaking][resource-leaking] docs on `TempDir`.
    ///
    /// # Errors
    ///
    /// If the directory can not be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let tmp_dir = Builder::new().tempdir()?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// [resource-leaking]: struct.TempDir.html#resource-leaking
    pub fn tempdir(&self) -> io::Result<TempDir> {
        self.tempdir_in(env::temp_dir())
    }
    /// Attempts to make a temporary directory inside of `dir`.
    /// The directory and everything inside it will be automatically
    /// deleted once the returned `TempDir` is destroyed.
    ///
    /// # Resource leaking
    ///
    /// See [the resource leaking][resource-leaking] docs on `TempDir`.
    ///
    /// # Errors
    ///
    /// If the directory can not be created, `Err` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use tempfile::Builder;
    ///
    /// let tmp_dir = Builder::new().tempdir_in("./")?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// [resource-leaking]: struct.TempDir.html#resource-leaking
    pub fn tempdir_in<P: AsRef<Path>>(&self, dir: P) -> io::Result<TempDir> {
        util::create_helper(
            dir.as_ref(),
            self.prefix,
            self.suffix,
            self.random_len,
            |path| dir::create(path, self.permissions.as_ref(), self.disable_cleanup),
        )
    }
    /// Attempts to create a temporary file (or file-like object) using the
    /// provided closure. The closure is passed a temporary file path and
    /// returns an [`std::io::Result`]. The path provided to the closure will be
    /// inside of [`env::temp_dir()`]. Use [`Builder::make_in`] to provide
    /// a custom temporary directory. If the closure returns one of the
    /// following errors, then another randomized file path is tried:
    ///  - [`std::io::ErrorKind::AlreadyExists`]
    ///  - [`std::io::ErrorKind::AddrInUse`]
    ///
    /// This can be helpful for taking full control over the file creation, but
    /// leaving the temporary file path construction up to the library. This
    /// also enables creating a temporary UNIX domain socket, since it is not
    /// possible to bind to a socket that already exists.
    ///
    /// Note that [`Builder::append`] is ignored when using [`Builder::make`].
    ///
    /// # Security
    ///
    /// This has the same [security implications][security] as
    /// [`NamedTempFile`], but with additional caveats. Specifically, it is up
    /// to the closure to ensure that the file does not exist and that such a
    /// check is *atomic*. Otherwise, a [time-of-check to time-of-use
    /// bug][TOCTOU] could be introduced.
    ///
    /// For example, the following is **not** secure:
    ///
    /// ```
    /// use std::fs::File;
    /// use tempfile::Builder;
    ///
    /// // This is NOT secure!
    /// let tempfile = Builder::new().make(|path| {
    ///     if path.is_file() {
    ///         return Err(std::io::ErrorKind::AlreadyExists.into());
    ///     }
    ///
    ///     // Between the check above and the usage below, an attacker could
    ///     // have replaced `path` with another file, which would get truncated
    ///     // by `File::create`.
    ///
    ///     File::create(path)
    /// })?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// Note that simply using [`std::fs::File::create`] alone is not correct
    /// because it does not fail if the file already exists:
    ///
    /// ```
    /// use tempfile::Builder;
    /// use std::fs::File;
    ///
    /// // This could overwrite an existing file!
    /// let tempfile = Builder::new().make(|path| File::create(path))?;
    /// # Ok::<(), std::io::Error>(())
    /// ```
    /// For creating regular temporary files, use [`Builder::tempfile`] instead
    /// to avoid these problems. This function is meant to enable more exotic
    /// use-cases.
    ///
    /// # Resource leaking
    ///
    /// See [the resource leaking][resource-leaking] docs on `NamedTempFile`.
    ///
    /// # Errors
    ///
    /// If the closure returns any error besides
    /// [`std::io::ErrorKind::AlreadyExists`] or
    /// [`std::io::ErrorKind::AddrInUse`], then `Err` is returned.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(unix)]
    /// # {
    /// use std::os::unix::net::UnixListener;
    /// use tempfile::Builder;
    ///
    /// let tempsock = Builder::new().make(|path| UnixListener::bind(path))?;
    /// # }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    ///
    /// [TOCTOU]: https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use
    /// [security]: struct.NamedTempFile.html#security
    /// [resource-leaking]: struct.NamedTempFile.html#resource-leaking
    pub fn make<F, R>(&self, f: F) -> io::Result<NamedTempFile<R>>
    where
        F: FnMut(&Path) -> io::Result<R>,
    {
        self.make_in(env::temp_dir(), f)
    }
    /// This is the same as [`Builder::make`], except `dir` is used as the base
    /// directory for the temporary file path.
    ///
    /// See [`Builder::make`] for more details and security implications.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(unix)]
    /// # {
    /// use tempfile::Builder;
    /// use std::os::unix::net::UnixListener;
    ///
    /// let tempsock = Builder::new().make_in("./", |path| UnixListener::bind(path))?;
    /// # }
    /// # Ok::<(), std::io::Error>(())
    /// ```
    pub fn make_in<F, R, P>(&self, dir: P, mut f: F) -> io::Result<NamedTempFile<R>>
    where
        F: FnMut(&Path) -> io::Result<R>,
        P: AsRef<Path>,
    {
        util::create_helper(
            dir.as_ref(),
            self.prefix,
            self.suffix,
            self.random_len,
            move |path| {
                Ok(NamedTempFile::from_parts(
                    f(&path)?,
                    TempPath::new(path, self.disable_cleanup),
                ))
            },
        )
    }
}
