macro_rules! deps {
    () => {
        Event!();
        Result!();
        Reader!();
    };
}

macro_rules! impl_1531 {
    () => {
        deps!();
        impl < 'buf , Fd : AsFd > Reader < 'buf , Fd > { # [doc = " Read the next inotify event."] # [doc = ""] # [doc = " This is similar to [`Iterator::next`] except that it doesn't return an"] # [doc = " `Option`, because the stream doesn't have an ending. It always returns"] # [doc = " events or errors."] # [doc = ""] # [doc = " If there are no events in the buffer and none ready to be read:"] # [doc = "  - If the file descriptor was opened with"] # [doc = "    [`inotify::CreateFlags::NONBLOCK`], this will fail with"] # [doc = "    [`Errno::AGAIN`]."] # [doc = "  - Otherwise this will block until at least one event is ready or an"] # [doc = "    error occurs."] # [allow (unsafe_code)] # [allow (clippy :: should_implement_trait)] pub fn next (& mut self) -> io :: Result < Event < '_ > > { if self . is_buffer_empty () { match read (self . fd . as_fd () , & mut * self . buf) . map (| (init , _) | init . len ()) { Ok (0) => return Err (Errno :: INVAL) , Ok (bytes_read) => { self . initialized = bytes_read ; self . offset = 0 ; } Err (e) => return Err (e) , } } let ptr = self . buf [self . offset ..] . as_ptr () ; let event = unsafe { & * ptr . cast :: < inotify_event > () } ; self . offset += size_of :: < inotify_event > () + usize :: try_from (event . len) . unwrap () ; Ok (Event { wd : event . wd , events : ReadFlags :: from_bits_retain (event . mask) , cookie : event . cookie , file_name : if event . len > 0 { Some (unsafe { CStr :: from_ptr (event . name . as_ptr () . cast ()) }) } else { None } , }) } # [doc = " Returns true if the internal buffer is empty and will be refilled when"] # [doc = " calling [`next`]. This is useful to avoid further blocking reads."] # [doc = ""] # [doc = " [`next`]: Self::next"] pub fn is_buffer_empty (& self) -> bool { self . offset >= self . initialized } }
    };
}

impl_1531!();