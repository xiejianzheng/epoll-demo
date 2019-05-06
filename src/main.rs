#![cfg_attr(test, deny(warnings))]
// Many of mio's public methods violate this lint, but they can't be fixed
// without a breaking change.
#![cfg_attr(feature = "cargo-clippy", allow(clippy::trivially_copy_pass_by_ref))]

extern crate iovec;
extern crate net2;
extern crate slab;

#[cfg(unix)]
extern crate libc;

#[macro_use]
extern crate log;

mod event_imp;
mod io;
mod lazycell;
mod poll;
mod sys;
mod token;

pub use event_imp::{PollOpt, Ready};
pub use poll::{Poll, Registration, SetReadiness};
pub use token::Token;

pub mod event {
    //! Readiness event types and utilities.

    pub use super::event_imp::{Event, Evented};
    pub use super::poll::{Events, Iter};
}

pub use event::Events;

pub mod unix {
    //! Unix only extensions
    pub use sys::unix::UnixReady;
    pub use sys::EventedFd;
}

fn main() {
}