use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A module for integrating signal handling with the MIO 0.8 runtime.
///
/// This provides the [`Signals`][v0_8::Signals] struct as an abstraction
/// which can be used with [`mio::Poll`][mio_0_8::Poll].
///
/// # Examples
///
/// ```rust
/// # use mio_0_8 as mio;
/// use std::io::{Error, ErrorKind};
///
/// use signal_hook::consts::signal::*;
/// use signal_hook_mio::v0_8::Signals;
///
/// use mio::{Events, Poll, Interest, Token};
///
/// fn main() -> Result<(), Error> {
///     let mut poll = Poll::new()?;
///
///     let mut signals = Signals::new(&[
///         SIGTERM,
/// #       SIGUSR1,
///     ])?;
///
///     let signal_token = Token(0);
///
///     poll.registry().register(&mut signals, signal_token, Interest::READABLE)?;
/// #   signal_hook::low_level::raise(SIGUSR1).unwrap(); // Just for terminating the example
///
///     let mut events = Events::with_capacity(10);
///     'outer: loop {
///         poll.poll(&mut events, None)
///             .or_else(|e| if e.kind() == ErrorKind::Interrupted {
///                 // We get interrupt when a signal happens inside poll. That's non-fatal, just
///                 // retry.
///                 events.clear();
///                 Ok(())
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
#[cfg(feature = "support-v0_8")]
pub mod v0_8 {
    use mio::event::Source;
    use mio::{Interest, Registry, Token};
    use mio_0_8 as mio;
    implement_signals_with_pipe!(mio::net::UnixStream);
    impl Source for Signals {
        fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interest: Interest,
        ) -> Result<(), Error> {
            self.0.get_read_mut().register(registry, token, interest)
        }
        fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interest: Interest,
        ) -> Result<(), Error> {
            self.0.get_read_mut().reregister(registry, token, interest)
        }
        fn deregister(&mut self, registry: &Registry) -> Result<(), Error> {
            self.0.get_read_mut().deregister(registry)
        }
    }
}
