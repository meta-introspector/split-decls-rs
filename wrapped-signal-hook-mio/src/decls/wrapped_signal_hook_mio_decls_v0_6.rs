use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A module for integrating signal handling with the MIO 0.6 runtime.
///
/// This provides the [`Signals`][v0_6::Signals] struct as an abstraction
/// which can be used with [`mio::Poll`][mio_0_6::Poll].
///
/// # Examples
///
/// ```rust
/// # use mio_0_6 as mio;
/// use std::io::{Error, ErrorKind};
///
/// use signal_hook::consts::signal::*;
/// use signal_hook_mio::v0_6::Signals;
///
/// use mio::{Events, Poll, PollOpt, Ready, Token};
///
/// fn main() -> Result<(), Error> {
///     let poll = Poll::new()?;
///
///     let mut signals = Signals::new(&[
///         SIGTERM,
/// #       SIGUSR1,
///     ])?;
///
///     let signal_token = Token(0);
///
///     poll.register(&mut signals, signal_token, Ready::readable(), PollOpt::level())?;
/// #   signal_hook::low_level::raise(SIGUSR1).unwrap(); // Just for terminating the example
///
///     let mut events = Events::with_capacity(10);
///     'outer: loop {
///         poll.poll(&mut events, None)
///             .or_else(|e| if e.kind() == ErrorKind::Interrupted {
///                 // We get interrupt when a signal happens inside poll. That's non-fatal, just
///                 // retry.
///                 events.clear();
///                 Ok(0)
///             } else {
///                 Err(e)
///             })?;
///         for event in events.iter() {
///             match event.token() {
///                 Token(0) => {
///                     for signal in signals.pending() {
///                         match signal {
///                             SIGTERM => break 'outer,
/// #                           SIGUSR1 => return Ok(()),
///                             _ => unreachable!(),
///                         }
///                     }
///                 },
///                 _ => unreachable!("Register other sources and match for their tokens here"),
///             }
///         }
///     }
///
///     Ok(())
/// }
/// ```
#[cfg(feature = "support-v0_6")]
pub mod v0_6 {
    use mio::event::Evented;
    use mio::{Poll, PollOpt, Ready, Token};
    use mio_0_6 as mio;
    implement_signals_with_pipe!(mio_uds::UnixStream);
    impl Evented for Signals {
        fn register(
            &self,
            poll: &Poll,
            token: Token,
            interest: Ready,
            opts: PollOpt,
        ) -> Result<(), Error> {
            self.0.get_read().register(poll, token, interest, opts)
        }
        fn reregister(
            &self,
            poll: &Poll,
            token: Token,
            interest: Ready,
            opts: PollOpt,
        ) -> Result<(), Error> {
            self.0.get_read().reregister(poll, token, interest, opts)
        }
        fn deregister(&self, poll: &Poll) -> Result<(), Error> {
            self.0.get_read().deregister(poll)
        }
    }
}
