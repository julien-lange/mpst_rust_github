use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::meshedchannels::MeshedChannels;

use mpstthree::role::a::RoleA;
use mpstthree::role::b::RoleB;
use mpstthree::role::c::RoleC;
use mpstthree::role::end::RoleEnd;

/// Creating the binary sessions
type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoC<N> = Send<N, End>;

type CtoA<N> = <AtoC<N> as Session>::Dual;
type CtoB<N> = <BtoC<N> as Session>::Dual;

/// Stacks
type StackA = RoleB<RoleC<RoleEnd>>;
type StackB = RoleA<RoleC<RoleEnd>>;
type StackC = RoleA<RoleB<RoleEnd>>;

/// Creating the MP sessions
type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
type EndpointB<N> = MeshedChannels<BtoA<N>, BtoC<N>, StackB, RoleB<RoleEnd>>;
type EndpointC<N> = MeshedChannels<CtoA<N>, CtoB<N>, StackC, RoleC<RoleEnd>>;

/////////////////////////////////////////

pub fn main() {
    let graphs =
        mpstthree::checker_concat!(EndpointA<i32>, EndpointC<i32>, EndpointB<i32>).unwrap();

    ////////////// Test graph A
    let graph_a = &graphs["RoleA"];

    assert_eq!(graph_a.node_count(), 4);
    assert_eq!(graph_a.edge_count(), 3);

    ////////////// Test graph B
    let graph_b = &graphs["RoleB"];

    assert_eq!(graph_b.node_count(), 4);
    assert_eq!(graph_b.edge_count(), 3);

    ////////////// Test graph C
    let graph_c = &graphs["RoleC"];

    assert_eq!(graph_c.node_count(), 4);
    assert_eq!(graph_c.edge_count(), 3);
}
