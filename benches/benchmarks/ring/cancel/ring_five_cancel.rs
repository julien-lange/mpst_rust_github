#![allow(dead_code)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpstthree::binary::struct_trait::{End, Recv, Send, Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    broadcast_cancel, bundle_struct_fork_close_multi,
    create_fn_choose_mpst_cancel_multi_to_all_bundle, create_multiple_normal_role_short,
    create_recv_mpst_session_bundle, create_send_check_cancel_bundle, offer_cancel_mpst,
};

use std::error::Error;
use std::time::Duration;

// Create the new SessionMpst for five participants and the close and fork functions
bundle_struct_fork_close_multi!(close_mpst_multi, fork_mpst, SessionMpstSix, 6);

// Create new roles
// normal
create_multiple_normal_role_short!(Central, A, B, C, D, E);

// Create new send functions
// A
create_send_check_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 2 | =>
    RoleA, SessionMpstSix, 6
);
// B
create_send_check_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 2 |
    send_mpst_b_to_c, RoleC, 3 | =>
    RoleB, SessionMpstSix, 6
);
// C
create_send_check_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 3 |
    send_mpst_c_to_d, RoleD, 4 | =>
    RoleC, SessionMpstSix, 6
);
// D
create_send_check_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 4 |
    send_mpst_d_to_e, RoleE, 5 | =>
    RoleD, SessionMpstSix, 6
);
// E
create_send_check_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 5 | =>
    RoleE, SessionMpstSix, 6
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 2 |
    recv_mpst_a_from_e, RoleE, 5 | =>
    RoleA, SessionMpstSix, 6
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 2 |
    recv_mpst_b_from_c, RoleC, 3 |
    recv_mpst_b_from_e, RoleE, 5 | =>
    RoleB, SessionMpstSix, 6
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 3 |
    recv_mpst_c_from_d, RoleD, 4 |
    recv_mpst_c_from_e, RoleE, 5 | =>
    RoleC, SessionMpstSix, 6
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 4 |
    recv_mpst_d_from_e, RoleE, 5 | =>
    RoleD, SessionMpstSix, 6
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 5 | =>
    RoleE, SessionMpstSix, 6
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;

// Types
// A
enum Branching0fromEtoA {
    Forward(SessionMpstSix<End, Send<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>),
    Backward(
        SessionMpstSix<End, Recv<(), End>, End, End, RecursAtoE, RoleB<RoleE<RoleEnd>>, NameA>,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoE = <Choose0fromEtoA as Session>::Dual;
// B
enum Branching0fromEtoB {
    Forward(
        SessionMpstSix<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursBtoE,
            RoleA<RoleC<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        SessionMpstSix<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursBtoE,
            RoleC<RoleA<RoleE<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoE = <Choose0fromEtoB as Session>::Dual;
// C
enum Branching0fromEtoC {
    Forward(
        SessionMpstSix<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursCtoE,
            RoleB<RoleD<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        SessionMpstSix<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursCtoE,
            RoleD<RoleB<RoleE<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoE = <Choose0fromEtoC as Session>::Dual;
// D
enum Branching0fromEtoD {
    Forward(
        SessionMpstSix<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursDtoE>,
            RoleC<RoleE<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        SessionMpstSix<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursDtoE>,
            RoleE<RoleC<RoleE<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(SessionMpstSix<End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoE = <Choose0fromEtoD as Session>::Dual;
// E
type Choose0fromEtoA = Send<(End, Branching0fromEtoA), End>;
type Choose0fromEtoB = Send<(End, Branching0fromEtoB), End>;
type Choose0fromEtoC = Send<(End, Branching0fromEtoC), End>;
type Choose0fromEtoD = Send<(End, Branching0fromEtoD), End>;
type EndpointDoneE = SessionMpstSix<End, End, End, End, End, RoleEnd, NameE>;
type EndpointForwardE = SessionMpstSix<
    End,
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Recv<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;
type EndpointBackwardE = SessionMpstSix<
    End,
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Send<(), Choose0fromEtoD>,
    RoleD<RoleBroadcast>,
    NameE,
>;

// Creating the MP sessions
type EndpointCentral = SessionMpstSix<End, End, End, End, End, RoleEnd, RoleCentral<RoleEnd>>;
type EndpointA = SessionMpstSix<End, End, End, End, RecursAtoE, RoleE<RoleEnd>, NameA>;
type EndpointB = SessionMpstSix<End, End, End, End, RecursBtoE, RoleE<RoleEnd>, NameB>;
type EndpointC = SessionMpstSix<End, End, End, End, RecursCtoE, RoleE<RoleEnd>, NameC>;
type EndpointD = SessionMpstSix<End, End, End, End, RecursDtoE, RoleE<RoleEnd>, NameD>;
type EndpointE = SessionMpstSix<
    End,
    Choose0fromEtoA,
    Choose0fromEtoB,
    Choose0fromEtoC,
    Choose0fromEtoD,
    RoleBroadcast,
    NameE,
>;

create_fn_choose_mpst_cancel_multi_to_all_bundle!(
    done_from_e_to_all, forward_from_e_to_all, backward_from_e_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneE, EndpointForwardE, EndpointBackwardE, =>
    Branching0fromEtoA,
    Branching0fromEtoB,
    Branching0fromEtoC,
    Branching0fromEtoD, =>
    RoleA, RoleB, RoleC, RoleD, =>
    RoleCentral, RoleE, SessionMpstSix, 6, 6
);

fn endpoint_central(s: EndpointCentral) -> Result<(), Box<dyn Error>> {
    broadcast_cancel!(s, RoleCentral, 6);
    Ok(())
}

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_a_from_e, {
        Branching0fromEtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromEtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_b_from_e, {
        Branching0fromEtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromEtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_c_from_e, {
        Branching0fromEtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromEtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_cancel_mpst!(s, recv_mpst_d_from_e, {
        Branching0fromEtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromEtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;
            endpoint_d(s)
        },
        Branching0fromEtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    recurs_e(s, SIZE)
}

fn recurs_e(s: EndpointE, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_e_to_all(s)?;

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_e_to_all(s)?;

            let (_, s) = recv_mpst_e_from_d(s)?;

            recurs_e(s, i - 1)
        }
        i => {
            let s = backward_from_e_to_all(s)?;

            let s = send_mpst_e_to_d((), s)?;

            recurs_e(s, i - 1)
        }
    }
}

fn all_mpst() -> Result<(), Box<dyn std::any::Any + std::marker::Send>> {
    let (thread_central, thread_a, thread_b, thread_c, thread_d, thread_e) = fork_mpst(
        black_box(endpoint_central),
        black_box(endpoint_a),
        black_box(endpoint_b),
        black_box(endpoint_c),
        black_box(endpoint_d),
        black_box(endpoint_e),
    );

    thread_central.join()?;
    thread_a.join()?;
    thread_b.join()?;
    thread_c.join()?;
    thread_d.join()?;
    thread_e.join()?;

    Ok(())
}

/////////////////////////

static SIZE: i64 = 100;

fn ring_five_mpst(c: &mut Criterion) {
    c.bench_function(&format!("ring five cancel protocol MPST {}", SIZE), |b| {
        b.iter(|| all_mpst())
    });
}

fn long_warmup() -> Criterion {
    Criterion::default().measurement_time(Duration::new(1800, 0))
}

criterion_group! {
    name = ring_five;
    // config = long_warmup();
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = ring_five_mpst
}
criterion_main!(ring_five);
