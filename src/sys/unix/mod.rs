mod epoll;

pub use self::epoll::{Events, Selector};

#[cfg(any(
    target_os = "bitrig",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
mod kqueue;

mod awakener;
mod eventedfd;
mod io;
mod ready;
mod tcp;
mod udp;
mod uio;

pub use self::awakener::Awakener;
pub use self::eventedfd::EventedFd;
pub use self::io::{set_nonblock, Io};
pub use self::ready::{UnixReady, READY_ALL};
pub use self::tcp::{TcpListener, TcpStream};
pub use self::udp::UdpSocket;

pub use iovec::IoVec;

use std::os::unix::io::FromRawFd;

pub fn pipe() -> ::io::Result<(Io, Io)> {
    // Use pipe2 for atomically setting O_CLOEXEC if we can, but otherwise
    // just fall back to using `pipe`.

    let mut pipes = [0; 2];
    let flags = libc::O_NONBLOCK | libc::O_CLOEXEC;
    unsafe {
        cvt(libc::pipe(pipes.as_mut_ptr()))?;
        libc::fcntl(pipes[0], libc::F_SETFL, flags);
        libc::fcntl(pipes[1], libc::F_SETFL, flags);
    }

    unsafe { Ok((Io::from_raw_fd(pipes[0]), Io::from_raw_fd(pipes[1]))) }
}

trait IsMinusOne {
    fn is_minus_one(&self) -> bool;
}

impl IsMinusOne for i32 {
    fn is_minus_one(&self) -> bool {
        *self == -1
    }
}
impl IsMinusOne for isize {
    fn is_minus_one(&self) -> bool {
        *self == -1
    }
}

fn cvt<T: IsMinusOne>(t: T) -> ::io::Result<T> {
    use std::io;

    if t.is_minus_one() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
