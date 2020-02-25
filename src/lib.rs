///#![feature(never_type)]
extern crate crossbeam_channel;
extern crate either;

use std::any::Any;
use std::error::Error;
use std::marker;
use std::panic;
use std::thread::{spawn, JoinHandle};

pub mod binary;
pub mod functionmpst;
pub mod role;
pub mod sessionmpst;

use binary::Session;
use role::Role;
use sessionmpst::SessionMpst;

pub fn fork_simple<S1, S2, R, P>(p: P, s: SessionMpst<S1, S2, R>) -> JoinHandle<()>
where
    S1: Session + 'static,
    S2: Session + 'static,
    R: Role + 'static,
    P: FnOnce(SessionMpst<S1, S2, R>) -> Result<(), Box<dyn Error>> + marker::Send + 'static,
{
    let other_thread = spawn(move || {
        panic::set_hook(Box::new(|_info| {
            // do nothing
        }));
        match p(s) {
            Ok(()) => (),
            Err(e) => panic!("{}", e.to_string()),
        }
    });
    other_thread
}

pub fn run_processes<S1, S2, S3, R1, R2, R3, F1, F2, F3>(
    f1: F1,
    f2: F2,
    f3: F3,
) -> (
    Result<(), Box<(dyn Any + marker::Send + 'static)>>,
    Result<(), Box<(dyn Any + marker::Send + 'static)>>,
    Result<(), Box<(dyn Any + marker::Send + 'static)>>,
)
where
    S1: Session + 'static,
    S2: Session + 'static,
    S3: Session + 'static,
    R1: Role + 'static,
    R2: Role + 'static,
    R3: Role + 'static,
    F1: FnOnce(SessionMpst<S1, <S3 as Session>::Dual, R1>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F2: FnOnce(SessionMpst<<S1 as Session>::Dual, S2, R2>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
    F3: FnOnce(SessionMpst<S3, <S2 as Session>::Dual, R3>) -> Result<(), Box<dyn Error>>
        + marker::Send
        + 'static,
{
    let (channel_ab, channel_ba) = S1::new();
    let (channel_ca, channel_ac) = S3::new();
    let (channel_bc, channel_cb) = S2::new();

    let (role_a, _) = R1::new();
    let (role_b, _) = R2::new();
    let (role_c, _) = R3::new();

    let a = SessionMpst {
        session1: channel_ab,
        session2: channel_ac,
        queue: role_a,
    };
    let b = SessionMpst {
        session1: channel_ba,
        session2: channel_bc,
        queue: role_b,
    };
    let c = SessionMpst {
        session1: channel_ca,
        session2: channel_cb,
        queue: role_c,
    };

    let thread_a = fork_simple(f1, a);
    let thread_b = fork_simple(f2, b);
    let thread_c = fork_simple(f3, c);

    (thread_a.join(), thread_b.join(), thread_c.join())
}
