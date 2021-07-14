// Test for parametrisation on the name of the roles
use mpstthree::binary::struct_trait::{End, Recv, Send};
use mpstthree::fork::fork_mpst;
use mpstthree::functionmpst::close::close_mpst;
use mpstthree::meshedchannels::MeshedChannels;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    create_multiple_normal_role, create_recv_mpst_session_1, create_recv_mpst_session_2,
    create_send_mpst_session_1, create_send_mpst_session_2,
};
use std::error::Error;

// Create new roles
create_multiple_normal_role!(
    RoleA, RoleADual |
    RoleB, RoleBDual |
    RoleD, RoleDDual |
);

type TestA = RoleA<RoleEnd>;
type TestB = RoleB<RoleEnd>;
type TestD = RoleD<RoleEnd>;

type SendMeshedChannelsD<N> = MeshedChannels<Send<N, End>, End, TestA, TestD>;

type SendMeshedChannelsA<N> = MeshedChannels<End, Send<N, End>, TestD, TestA>;

type RecvMeshedChannelsD<N> = MeshedChannels<Recv<N, End>, End, TestA, TestD>;

type RecvMeshedChannelsA<N> = MeshedChannels<End, Recv<N, End>, TestD, TestA>;

// Create an B dummy
type Dummy = MeshedChannels<End, End, RoleEnd, TestB>;

// Create new send functions
create_send_mpst_session_1!(send_mpst_d_to_a, RoleA, RoleD);
create_send_mpst_session_2!(send_mpst_a_to_d, RoleD, RoleA);

// Create new recv functions
create_recv_mpst_session_1!(recv_mpst_d_from_a, RoleA, RoleD);
create_recv_mpst_session_2!(recv_mpst_a_from_d, RoleD, RoleA);

// The functions for the basic exchanges
fn send_a_to_d(s: SendMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_a_to_d(0, s);
    close_mpst(s)
}

fn send_d_to_a(s: SendMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_d_to_a(0, s);
    close_mpst(s)
}

fn recv_a_to_d(s: RecvMeshedChannelsA<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_a_from_d(s)?;
    close_mpst(s)
}

fn recv_d_to_a(s: RecvMeshedChannelsD<i32>) -> Result<(), Box<dyn Error>> {
    let (_, s) = recv_mpst_d_from_a(s)?;
    close_mpst(s)
}

fn dummy(s: Dummy) -> Result<(), Box<dyn Error>> {
    close_mpst(s)
}

/////////////////////////////////////////

pub fn basic_macros_send() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(send_a_to_d, dummy, recv_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}

pub fn basic_macros_recv() {
    assert!(|| -> Result<(), Box<dyn Error>> {
        {
            let (thread_a, thread_dummy, thread_d) = fork_mpst(recv_a_to_d, dummy, send_d_to_a);

            assert!(thread_a.join().is_ok());
            assert!(thread_dummy.join().is_ok());
            assert!(thread_d.join().is_ok());
        }
        Ok(())
    }()
    .is_ok());
}