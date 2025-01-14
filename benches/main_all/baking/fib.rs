use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::bundle_impl_with_enum_and_cancel;
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create new MeshedChannels for four participants
bundle_impl_with_enum_and_cancel!(MeshedChannelsThree, A, B, C);

// Names
type NameA = RoleA<RoleEnd>;
type NameB = RoleB<RoleEnd>;
type NameC = RoleC<RoleEnd>;

// Types
// A
type Choose0fromAtoB = <RecursBtoA as Session>::Dual;
type Choose0fromAtoC = <RecursCtoA as Session>::Dual;
// B
enum Branching0fromAtoB {
    More(
        MeshedChannelsThree<
            Recv<i64, Send<i64, RecursBtoA>>,
            End,
            RoleA<RoleA<RoleA<RoleEnd>>>,
            NameB,
        >,
    ),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameB>),
}
type RecursBtoA = Recv<Branching0fromAtoB, End>;
// C
enum Branching0fromAtoC {
    More(MeshedChannelsThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>),
    Done(MeshedChannelsThree<End, End, RoleEnd, NameC>),
}
type RecursCtoA = Recv<Branching0fromAtoC, End>;

// Creating the MP sessions
type EndpointA = MeshedChannelsThree<Choose0fromAtoB, Choose0fromAtoC, RoleBroadcast, NameA>;
type EndpointAMore = MeshedChannelsThree<
    Send<i64, Recv<i64, Choose0fromAtoB>>,
    Choose0fromAtoC,
    RoleB<RoleB<RoleBroadcast>>,
    NameA,
>;
type EndpointB = MeshedChannelsThree<RecursBtoA, End, RoleA<RoleEnd>, NameB>;
type EndpointC = MeshedChannelsThree<RecursCtoA, End, RoleA<RoleEnd>, NameC>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    recurs_a(s, LOOPS, 1)
}

fn recurs_a(s: EndpointA, index: i64, old: i64) -> Result<(), Box<dyn Error>> {
    match index {
        0 => {
            let s = choose_mpst_a_to_all!(s, Branching0fromAtoB::Done, Branching0fromAtoC::Done);

            s.close()
        }
        i => {
            let s: EndpointAMore =
                choose_mpst_a_to_all!(s, Branching0fromAtoB::More, Branching0fromAtoC::More);

            let s = s.send(old)?;
            let (new, s) = s.recv()?;

            recurs_a(s, i - 1, new)
        }
    }
}

fn endpoint_b(s: EndpointB) -> Result<(), Box<dyn Error>> {
    recurs_b(s, 0)
}

fn recurs_b(s: EndpointB, old: i64) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoB::Done(s) => {
            s.close()
        },
        Branching0fromAtoB::More(s) => {
            let (new, s) = s.recv()?;
            let s = s.send(new + old)?;
            recurs_b(s, new + old)
        },
    })
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromAtoC::Done(s) => {
            s.close()
        },
        Branching0fromAtoC::More(s) => {
            endpoint_c(s)
        },
    })
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

static LOOPS: i64 = 20;

fn fibo_mpst(c: &mut Criterion) {
    c.bench_function(&format!("Fibo MPST {} baking", LOOPS), |b| b.iter(all_mpst));
}

criterion_group! {
    name = fib;
    config = Criterion::default().significance_level(0.1).sample_size(10100);
    targets = fibo_mpst
}

criterion_main!(fib);
