use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::time::Duration;
use std::{cmp, i32};

use libc::{self, c_int};
use libc::{EPOLLERR, EPOLLHUP, EPOLLRDHUP, EPOLLONESHOT, EPOLL_CLOEXEC};
use libc::{EPOLLET, EPOLLOUT, EPOLLIN, EPOLLPRI};

use {io, Ready, PollOpt, Token};
use event_impl::Event;
use sys::unix::{cvt, UnixReady};
use sys::unix::io::set_cloexec;

static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;

#[derive(Debug)]
pub struct Selector {
    id: usize,
    epfd: RawFd,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let epfd = unsafe {
            libc::epoll_create1(EPOLL_CLOEXEC);
        };

        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed) + 1;

        OK(Selector {
            id: id,
            epfd: epfd,
        })
    }

    pub fn id(&self) ->usize {
        self.id
    }

    pub fn select(&self, evts: &mut Events, awakener: Token, timeout: Option<Duration>) -> io::Result<bool> {
        let timeout_ms = timeout
            .map(|to| cmp::min(millis(to), i32::MAX as u64) as i32)
            .unwrap_or(-1);

        evts.clear();
        unsafe {
            let cnt = cvt(libc::epoll_wait(self.epfd,
                                           evts.events.as_mut_ptr(),
                                           evts.events.capacity() as i32,
                                           timeout_ms))?;
            let cnt = cnt as usize;
            evts.events.set_len(cnt);
            
            for i in 0..cnt {
                if evts.events[i].u64 as usize == awakener.info() {
                    evts.events.remove(i);
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Register event interests for the given IO handle with the OS 
    pub fn register(&self, fd: RawFd, token: Token, interests: Ready, opts: PollOpt) -> io::Result<()> {
        let mut info = libc::epoll_event {
            events: ioevent_to_epoll(interests, opts),
            u64: usize::from(token) as u64
        };

        unsafe {
            cvt(libc::epoll_ctl(self.epfd, libc::EPOLL_CTL_ADD, fd, &mut info))?;
            Ok(())
        }
    }
    
    
    /// Register event interests for the given IO handle with the OS
    pub fn reregister(&self, fd: RawFd, token: Token, interests: Ready, opts: PollOpt) -> io::Result<()> {
        let mut info = libc::epoll_event{
            events: ioevent_to_epoll(interests, opts),
            u64: usize::from(token) as u64
        };

        unsafe {
            cvt(libc::epoll_ctl(self.epfd, libc::EPOLL_CTL_MOD, fd, &mut info))?;
            Ok(())
        }
    }

    /// Deregister event interests for the given IO handle with the OS
    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        // The &info argument should be ignored by the system,
        // but linux < 2.6.9 required it to be not null.
        // For compatibility, we provide a dummy EpollEvent.
        let mut info = libc::epoll_event {
            events: 0,
            u64: 0,
        };

        unsafe {
            cvt(libc::epoll_ctl(self.epfd, libc::EPOLL_CTL_DEL, fd, &mut info))?;
            Ok(())
        }
    }

    fn ioevent_to_epoll(interest: Ready, opts: PollOpt) -> u32 {
        let mut kind = 0;

        if interest.is_readable() {
            kind |= EPOLLIN; 
        }

        if interest.is_writable() {
            kind |= EPOLLOUT;  
        }

        if UnixReady::from(interest).is_hup() {
            kind |= EPOLLHUP;
        }

        if opts.is_edge() {
            kind |= EPOLLET;
        }

        if opts.is_oneshot() {
            kind |= EPOLLONESHOT;
        }

        if opts.is_level() {
            kind &= !EPOLLET;
        }

        kind as u32
    }

    impl AsRawFd for Selector {
        fn as_raw_fd(&self) -> RawFd {
            self.epfd
        }

    }

    impl Drop for Selector {
        fn drop(&mut self) {
            unsafe {
                let _ = libc::close(self.epfd);
            }
        }
    }

    pub struct Events {
        events: Vec<libc::epoll_event>,
    }

    impl Events {
        pub fn with_capacity(u: usize) -> Events {
            Events {
                events: Vec::with_capacity(u)
            }
        }

        #[inline]
        pub fn capacity(&self) -> usize {
            self.events.capacity()
        }

        #[inline]
        pub fn is_empty(&self) -> bool {
            self.events.is_empty()
        }

        #[inline]
        pub fn get(&self, idx: usize) -> Option<Event> {

        }
























































}
