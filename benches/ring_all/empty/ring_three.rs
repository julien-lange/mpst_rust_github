use crossbeam_channel::bounded;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::close::close;
use mpstthree::binary::fork::fork_with_thread_id;
use mpstthree::binary::recv::recv;
use mpstthree::binary::send::send;
use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi, choose, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_session_bundle, offer, offer_mpst,
};

use std::error::Error;
use std::thread::{spawn, JoinHandle};
// use std::time::Duration;

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, MeshedChannelsThree, 3);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C);

// Create new send functions
// A
create_send_mpst_session_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsThree, 3
);
// B
create_send_mpst_session_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    RoleB, MeshedChannelsThree, 3
);
// C
create_send_mpst_session_bundle!(
    send_mpst_c_to_b, RoleB, 2 | =>
    RoleC, MeshedChannelsThree, 3
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_c, RoleC, 2 | =>
    RoleA, MeshedChannelsThree, 3
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 | =>
    RoleB, MeshedChannelsThree, 3
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 | =>
    RoleC, MeshedChannelsThree, 3
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
enum Branching0fromCtoA {
    Forward(MeshedChannelsThree<Send<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Backward(MeshedChannelsThree<Recv<(), End>, RecursAtoC, RoleB<RoleC<RoleEnd>>, NameA>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameA>),
}
type RecursAtoC = <Choose0fromCtoA as Session>::Dual;
// B
enum Branching0fromCtoB {
    Forward(
        MeshedChannelsThree<
            Recv<(), End>,
            Send<(), RecursBtoC>,
            RoleA<RoleC<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsThree<
            Send<(), End>,
            Recv<(), RecursBtoC>,
            RoleC<RoleA<RoleC<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoC = <Choose0fromCtoB as Session>::Dual;
// C
type Choose0fromCtoA = Send<Branching0fromCtoA, End>;
type Choose0fromCtoB = Send<Branching0fromCtoB, End>;
type EndpointDoneC = MeshedChannelsThree<End, End, RoleEnd, NameC>;
type EndpointForwardC =
    MeshedChannelsThree<Choose0fromCtoA, Recv<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;
type EndpointBackwardC =
    MeshedChannelsThree<Choose0fromCtoA, Send<(), Choose0fromCtoB>, RoleB<RoleBroadcast>, NameC>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<End, RecursAtoC, RoleC<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsThree<End, RecursBtoC, RoleC<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<Choose0fromCtoA, Choose0fromCtoB, RoleBroadcast, NameC>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_c_to_all, forward_from_c_to_all, backward_from_c_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneC, EndpointForwardC, EndpointBackwardC, =>
    Branching0fromCtoA, Branching0fromCtoB, =>
    RoleA, RoleB, =>
    RoleC, MeshedChannelsThree, 3
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_c, {
        Branching0fromCtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s);
            endpoint_a(s)
        },
        Branching0fromCtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_c, {
        Branching0fromCtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromCtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s);
            endpoint_b(s)
        },
        Branching0fromCtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s);
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    recurs_c(s, LOOPS)
}

fn recurs_c(s: EndpointC, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_c_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_c_to_all(s);

            let (_, s) = recv_mpst_c_from_b(s)?;

            recurs_c(s, i - 1)
        }
        i => {
            let s = backward_from_c_to_all(s);

            let s = send_mpst_c_to_b((), s);

            recurs_c(s, i - 1)
        }
    }
}

fn all_mpst() {
    let (thread_a, thread_b, thread_c) = fork_mpst(
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
    );

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
}

/////////////////////////
// A
enum BinaryA {
    More(Recv<(), Send<(), RecursA>>),
    Done(End),
}
type RecursA = Recv<BinaryA, End>;
fn binary_a_to_b(s: RecursA) -> Result<(), Box<dyn Error>> {
    offer!(s, {
        BinaryA::Done(s) => {
            close(s)
        },
        BinaryA::More(s) => {
            let (_, s) = recv(s)?;
            let s = send((), s);
            binary_a_to_b(s)
        },
    })
}

// B
type RecursB = <RecursA as Session>::Dual;
fn binary_b_to_a(s: Send<(), Recv<(), RecursB>>) -> Result<RecursB, Box<dyn Error>> {
    let s = send((), s);
    let (_, s) = recv(s)?;
    Ok(s)
}

fn all_binaries() {
    let mut threads = Vec::new();
    let mut sessions = Vec::new();

    for _ in 0..2 {
        let (thread, s): (JoinHandle<()>, RecursB) = fork_with_thread_id(black_box(binary_a_to_b));

        threads.push(thread);
        sessions.push(s);
    }

    let main = spawn(move || {
        for _ in 0..LOOPS {
            sessions = sessions
                .into_iter()
                .map(|s| binary_b_to_a(choose!(BinaryA::More, s)).unwrap())
                .collect::<Vec<_>>();
        }

        sessions
            .into_iter()
            .for_each(|s| close(choose!(BinaryA::Done, s)).unwrap());

        threads.into_iter().for_each(|elt| elt.join().unwrap());
    });

    main.join().unwrap();
}

/////////////////////////

type ReceivingSendingReceiving = crossbeam_channel::Receiver<SendingReceiving>;
type SendingReceivingSending = crossbeam_channel::Sender<ReceivingSending>;

type SendingReceiving = crossbeam_channel::Sender<Receiving>;
type ReceivingSending = crossbeam_channel::Receiver<Sending>;

type Receiving = crossbeam_channel::Receiver<()>;
type Sending = crossbeam_channel::Sender<()>;

fn all_crossbeam() {
    let mut threads = Vec::new();

    for _ in 0..2 {
        let main = spawn(move || {
            for _ in 0..LOOPS {
                let (sender_0, receiver_0) = bounded::<ReceivingSendingReceiving>(1);
                let (sender_4, receiver_4) = bounded::<SendingReceivingSending>(1);

                let (sender_1, receiver_1) = bounded::<SendingReceiving>(1);
                let (sender_5, receiver_5) = bounded::<ReceivingSending>(1);

                let (sender_2, receiver_2) = bounded::<Receiving>(1);
                let (sender_6, receiver_6) = bounded::<Sending>(1);

                let (sender_3, receiver_3) = bounded::<()>(1);
                let (sender_7, receiver_7) = bounded::<()>(1);

                sender_0.send(receiver_1).unwrap();
                sender_4.send(sender_5).unwrap();

                let receiver_1_bis = receiver_0.recv().unwrap();
                let sender_5_bis = receiver_4.recv().unwrap();

                sender_1.send(sender_2).unwrap();
                sender_5_bis.send(receiver_6).unwrap();

                let sender_2_bis = receiver_1_bis.recv().unwrap();
                let receiver_6_bis = receiver_5.recv().unwrap();

                sender_2_bis.send(receiver_3).unwrap();
                sender_6.send(sender_7).unwrap();

                let receiver_2_bis = receiver_2.recv().unwrap();
                let sender_7_bis = receiver_6_bis.recv().unwrap();

                sender_3.send(()).unwrap();
                sender_7_bis.send(()).unwrap();

                receiver_2_bis.recv().unwrap();
                receiver_7.recv().unwrap();
            }

            // "Close" connection
            let (sender_close_1, receiver_close_1) = bounded::<()>(1);
            let (sender_close_2, receiver_close_2) = bounded::<()>(1);

            sender_close_1.send(()).unwrap_or(());
            sender_close_2.send(()).unwrap_or(());

            receiver_close_1.recv().unwrap_or(());
            receiver_close_2.recv().unwrap_or(());
        });

        threads.push(main);
    }

    threads.into_iter().for_each(|elt| elt.join().unwrap());
}

/////////////////////////

static LOOPS: i64 = 0;

fn ring_protocol_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring three empty protocol MPST {}", LOOPS), |b| {
        b.iter(all_mpst)
    });
}

fn ring_protocol_binary(c: &mut Criterion) {
    c.bench_function(
        &format!("ring three empty protocol binary {}", LOOPS),
        |b| b.iter(all_binaries),
    );
}

fn ring_protocol_crossbeam(c: &mut Criterion) {
    c.bench_function(
        &format!("ring three empty protocol crossbeam {}", LOOPS),
        |b| b.iter(all_crossbeam),
    );
}

criterion_group! {
    name = ring_three;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ring_protocol_mpst, ring_protocol_binary, ring_protocol_crossbeam
}

criterion_main!(ring_three);
