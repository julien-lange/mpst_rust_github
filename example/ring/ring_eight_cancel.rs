use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{
    bundle_struct_fork_close_multi_cancel, create_fn_choose_mpst_multi_to_all_bundle,
    create_multiple_normal_role_short, create_recv_mpst_session_bundle,
    create_send_mpst_cancel_bundle, offer_mpst,
};

use std::error::Error;

// Create the new MeshedChannels for eight participants and the close and fork functions
bundle_struct_fork_close_multi_cancel!(close_mpst_multi, fork_mpst, MeshedChannelsEight, 8);

// Create new roles
// normal
create_multiple_normal_role_short!(A, B, C, D, E, F, G, H);

// Create new send functions
// A
create_send_mpst_cancel_bundle!(
    send_mpst_a_to_b, RoleB, 1 | =>
    RoleA, MeshedChannelsEight, 8
);
// B
create_send_mpst_cancel_bundle!(
    send_mpst_b_to_a, RoleA, 1 |
    send_mpst_b_to_c, RoleC, 2 | =>
    RoleB, MeshedChannelsEight, 8
);
// C
create_send_mpst_cancel_bundle!(
    send_mpst_c_to_b, RoleB, 2 |
    send_mpst_c_to_d, RoleD, 3 | =>
    RoleC, MeshedChannelsEight, 8
);
// D
create_send_mpst_cancel_bundle!(
    send_mpst_d_to_c, RoleC, 3 |
    send_mpst_d_to_e, RoleE, 4 | =>
    RoleD, MeshedChannelsEight, 8
);
// E
create_send_mpst_cancel_bundle!(
    send_mpst_e_to_d, RoleD, 4 |
    send_mpst_e_to_f, RoleF, 5 | =>
    RoleE, MeshedChannelsEight, 8
);
// F
create_send_mpst_cancel_bundle!(
    send_mpst_f_to_e, RoleE, 5 |
    send_mpst_f_to_g, RoleG, 6 | =>
    RoleF, MeshedChannelsEight, 8
);
// G
create_send_mpst_cancel_bundle!(
    send_mpst_g_to_f, RoleF, 6 |
    send_mpst_g_to_h, RoleH, 7 | =>
    RoleG, MeshedChannelsEight, 8
);
// H
create_send_mpst_cancel_bundle!(
    send_mpst_h_to_g, RoleG, 7 | =>
    RoleH, MeshedChannelsEight, 8
);

// Create new recv functions and related types
// A
create_recv_mpst_session_bundle!(
    recv_mpst_a_from_b, RoleB, 1 |
    recv_mpst_a_from_h, RoleH, 7 | =>
    RoleA, MeshedChannelsEight, 8
);
// B
create_recv_mpst_session_bundle!(
    recv_mpst_b_from_a, RoleA, 1 |
    recv_mpst_b_from_c, RoleC, 2 |
    recv_mpst_b_from_h, RoleH, 7 | =>
    RoleB, MeshedChannelsEight, 8
);
// C
create_recv_mpst_session_bundle!(
    recv_mpst_c_from_b, RoleB, 2 |
    recv_mpst_c_from_d, RoleD, 3 |
    recv_mpst_c_from_h, RoleH, 7 | =>
    RoleC, MeshedChannelsEight, 8
);
// D
create_recv_mpst_session_bundle!(
    recv_mpst_d_from_c, RoleC, 3 |
    recv_mpst_d_from_e, RoleE, 4 |
    recv_mpst_d_from_h, RoleH, 7 | =>
    RoleD, MeshedChannelsEight, 8
);
// E
create_recv_mpst_session_bundle!(
    recv_mpst_e_from_d, RoleD, 4 |
    recv_mpst_e_from_f, RoleF, 5 |
    recv_mpst_e_from_h, RoleH, 7 | =>
    RoleE, MeshedChannelsEight, 8
);
// F
create_recv_mpst_session_bundle!(
    recv_mpst_f_from_e, RoleE, 5 |
    recv_mpst_f_from_g, RoleG, 6 |
    recv_mpst_f_from_h, RoleH, 7 | =>
    RoleF, MeshedChannelsEight, 8
);
// G
create_recv_mpst_session_bundle!(
    recv_mpst_g_from_f, RoleF, 6 |
    recv_mpst_g_from_h, RoleH, 7 | =>
    RoleG, MeshedChannelsEight, 8
);
// H
create_recv_mpst_session_bundle!(
    recv_mpst_h_from_g, RoleG, 7 | =>
    RoleH, MeshedChannelsEight, 8
);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameD = RoleD<RoleEnd>;
type NameE = RoleE<RoleEnd>;
type NameF = RoleF<RoleEnd>;
type NameG = RoleG<RoleEnd>;
type NameH = RoleH<RoleEnd>;

// Types
// A
enum Branching0fromHtoA {
    Forward(
        MeshedChannelsEight<
            Send<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursAtoH,
            RoleB<RoleH<RoleEnd>>,
            NameA,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            Recv<(), End>,
            End,
            End,
            End,
            End,
            End,
            RecursAtoH,
            RoleB<RoleH<RoleEnd>>,
            NameA,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameA>),
}
type RecursAtoH = <Choose0fromHtoA as Session>::Dual;
// B
enum Branching0fromHtoB {
    Forward(
        MeshedChannelsEight<
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            End,
            RecursBtoH,
            RoleA<RoleC<RoleH<RoleEnd>>>,
            NameB,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            End,
            RecursBtoH,
            RoleC<RoleA<RoleH<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameB>),
}
type RecursBtoH = <Choose0fromHtoB as Session>::Dual;
// C
enum Branching0fromHtoC {
    Forward(
        MeshedChannelsEight<
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleB<RoleD<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            End,
            RecursCtoH,
            RoleD<RoleB<RoleH<RoleEnd>>>,
            NameC,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameC>),
}
type RecursCtoH = <Choose0fromHtoC as Session>::Dual;
// D
enum Branching0fromHtoD {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            End,
            RecursDtoH,
            RoleC<RoleE<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            End,
            RecursDtoH,
            RoleE<RoleC<RoleH<RoleEnd>>>,
            NameD,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameD>),
}
type RecursDtoH = <Choose0fromHtoD as Session>::Dual;
// E
enum Branching0fromHtoE {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            End,
            RecursEtoH,
            RoleD<RoleF<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            End,
            RecursEtoH,
            RoleF<RoleD<RoleH<RoleEnd>>>,
            NameE,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameE>),
}
type RecursEtoH = <Choose0fromHtoE as Session>::Dual;
// F
enum Branching0fromHtoF {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), End>,
            RecursFtoH,
            RoleE<RoleG<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), End>,
            RecursFtoH,
            RoleG<RoleE<RoleH<RoleEnd>>>,
            NameF,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameF>),
}
type RecursFtoH = <Choose0fromHtoF as Session>::Dual;
// G
enum Branching0fromHtoG {
    Forward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            End,
            Recv<(), End>,
            Send<(), RecursGtoH>,
            RoleF<RoleH<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Backward(
        MeshedChannelsEight<
            End,
            End,
            End,
            End,
            End,
            Send<(), End>,
            Recv<(), RecursGtoH>,
            RoleH<RoleF<RoleH<RoleEnd>>>,
            NameG,
        >,
    ),
    Done(MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameG>),
}
type RecursGtoH = <Choose0fromHtoG as Session>::Dual;
// H
type Choose0fromHtoA = Send<Branching0fromHtoA, End>;
type Choose0fromHtoB = Send<Branching0fromHtoB, End>;
type Choose0fromHtoC = Send<Branching0fromHtoC, End>;
type Choose0fromHtoD = Send<Branching0fromHtoD, End>;
type Choose0fromHtoE = Send<Branching0fromHtoE, End>;
type Choose0fromHtoF = Send<Branching0fromHtoF, End>;
type Choose0fromHtoG = Send<Branching0fromHtoG, End>;
type EndpointDoneH = MeshedChannelsEight<End, End, End, End, End, End, End, RoleEnd, NameH>;
type EndpointForwardH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Recv<(), Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;
type EndpointBackwardH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Send<(), Choose0fromHtoG>,
    RoleG<RoleBroadcast>,
    NameH,
>;

// Creating the MP sessions
type EndpointA = MeshedChannelsEight<End, End, End, End, End, End, RecursAtoH, RoleH<RoleEnd>, NameA>;
type EndpointB = MeshedChannelsEight<End, End, End, End, End, End, RecursBtoH, RoleH<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsEight<End, End, End, End, End, End, RecursCtoH, RoleH<RoleEnd>, NameC>;
type EndpointD = MeshedChannelsEight<End, End, End, End, End, End, RecursDtoH, RoleH<RoleEnd>, NameD>;
type EndpointE = MeshedChannelsEight<End, End, End, End, End, End, RecursEtoH, RoleH<RoleEnd>, NameE>;
type EndpointF = MeshedChannelsEight<End, End, End, End, End, End, RecursFtoH, RoleH<RoleEnd>, NameF>;
type EndpointG = MeshedChannelsEight<End, End, End, End, End, End, RecursGtoH, RoleH<RoleEnd>, NameG>;
type EndpointH = MeshedChannelsEight<
    Choose0fromHtoA,
    Choose0fromHtoB,
    Choose0fromHtoC,
    Choose0fromHtoD,
    Choose0fromHtoE,
    Choose0fromHtoF,
    Choose0fromHtoG,
    RoleBroadcast,
    NameH,
>;

create_fn_choose_mpst_multi_to_all_bundle!(
    done_from_h_to_all, forward_from_h_to_all, backward_from_h_to_all, =>
    Done, Forward, Backward, =>
    EndpointDoneH, EndpointForwardH, EndpointBackwardH, =>
    Branching0fromHtoA,
    Branching0fromHtoB,
    Branching0fromHtoC,
    Branching0fromHtoD,
    Branching0fromHtoE,
    Branching0fromHtoF,
    Branching0fromHtoG, =>
    RoleA, RoleB, RoleC, RoleD, RoleE, RoleF, RoleG, =>
    RoleH, MeshedChannelsEight, 8
);

fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_a_from_h, {
        Branching0fromHtoA::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoA::Forward(s) => {
            let s = send_mpst_a_to_b((), s)?;
            endpoint_a(s)
        },
        Branching0fromHtoA::Backward(s) => {
            let (_, s) = recv_mpst_a_from_b(s)?;
            endpoint_a(s)
        },
    })
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_b_from_h, {
        Branching0fromHtoB::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoB::Forward(s) => {
            let ((), s) = recv_mpst_b_from_a(s)?;
            let s = send_mpst_b_to_c((), s)?;
            endpoint_b(s)
        },
        Branching0fromHtoB::Backward(s) => {
            let ((), s) = recv_mpst_b_from_c(s)?;
            let s = send_mpst_b_to_a((), s)?;
            endpoint_b(s)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_c_from_h, {
        Branching0fromHtoC::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoC::Forward(s) => {
            let ((), s) = recv_mpst_c_from_b(s)?;
            let s = send_mpst_c_to_d((), s)?;
            endpoint_c(s)
        },
        Branching0fromHtoC::Backward(s) => {
            let ((), s) = recv_mpst_c_from_d(s)?;
            let s = send_mpst_c_to_b((), s)?;
            endpoint_c(s)
        },
    })
}

fn endpoint_d(s: EndpointD) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_d_from_h, {
        Branching0fromHtoD::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoD::Forward(s) => {
            let ((), s) = recv_mpst_d_from_c(s)?;
            let s = send_mpst_d_to_e((), s)?;
            endpoint_d(s)
        },
        Branching0fromHtoD::Backward(s) => {
            let ((), s) = recv_mpst_d_from_e(s)?;
            let s = send_mpst_d_to_c((), s)?;
            endpoint_d(s)
        },
    })
}

fn endpoint_e(s: EndpointE) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_e_from_h, {
        Branching0fromHtoE::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoE::Forward(s) => {
            let ((), s) = recv_mpst_e_from_d(s)?;
            let s = send_mpst_e_to_f((), s)?;
            endpoint_e(s)
        },
        Branching0fromHtoE::Backward(s) => {
            let ((), s) = recv_mpst_e_from_f(s)?;
            let s = send_mpst_e_to_d((), s)?;
            endpoint_e(s)
        },
    })
}

fn endpoint_f(s: EndpointF) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_f_from_h, {
        Branching0fromHtoF::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoF::Forward(s) => {
            let ((), s) = recv_mpst_f_from_e(s)?;
            let s = send_mpst_f_to_g((), s)?;
            endpoint_f(s)
        },
        Branching0fromHtoF::Backward(s) => {
            let ((), s) = recv_mpst_f_from_g(s)?;
            let s = send_mpst_f_to_e((), s)?;
            endpoint_f(s)
        },
    })
}

fn endpoint_g(s: EndpointG) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, recv_mpst_g_from_h, {
        Branching0fromHtoG::Done(s) => {
            close_mpst_multi(s)
        },
        Branching0fromHtoG::Forward(s) => {
            let ((), s) = recv_mpst_g_from_f(s)?;
            let s = send_mpst_g_to_h((), s)?;
            endpoint_g(s)
        },
        Branching0fromHtoG::Backward(s) => {
            let ((), s) = recv_mpst_g_from_h(s)?;
            let s = send_mpst_g_to_f((), s)?;
            endpoint_g(s)
        },
    })
}

fn endpoint_h(s: EndpointH) -> Result<(), Box<dyn Error>> {
    recurs_h(s, 100)
}

fn recurs_h(s: EndpointH, index: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = done_from_h_to_all(s);

            close_mpst_multi(s)
        }
        i if i % 2 == 0 => {
            let s = forward_from_h_to_all(s);

            let (_, s) = recv_mpst_h_from_g(s)?;

            recurs_h(s, i - 1)
        }
        i => {
            let s = backward_from_h_to_all(s);

            let s = send_mpst_h_to_g((), s)?;

            recurs_h(s, i - 1)
        }
    }
}

fn main() {
    let (thread_a, thread_b, thread_c, thread_d, thread_e, thread_f, thread_g, thread_h) =
        fork_mpst(
            endpoint_a, endpoint_b, endpoint_c, endpoint_d, endpoint_e, endpoint_f, endpoint_g,
            endpoint_h,
        );

    assert!(thread_a.join().is_ok());
    assert!(thread_b.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_d.join().is_ok());
    assert!(thread_e.join().is_ok());
    assert!(thread_f.join().is_ok());
    assert!(thread_g.join().is_ok());
    assert!(thread_h.join().is_ok());
}
