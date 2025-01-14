#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send, session::Session};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, checker_concat};

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

bundle_impl_with_enum_and_cancel!(MeshedChannelsThree, A, C, S);

// Payloads
struct Start;
struct Redirect;
struct Login;
struct Auth;
struct Password;
struct Success;
struct Token;
struct Fail;
struct Received;

// Names
type NameA = RoleA<RoleEnd>;
type NameC = RoleC<RoleEnd>;
type NameS = RoleS<RoleEnd>;

// Types

// A
type Choose0fromAtoC = <Offer0fromCtoA as Session>::Dual;
type Choose0fromAtoS = <Offer0fromStoA as Session>::Dual;

// C
enum Branching0fromAtoC {
    Success(
        MeshedChannelsThree<
            Recv<Success, End>,
            Send<Success, Recv<Token, End>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
    Fail(
        MeshedChannelsThree<
            Recv<Fail, End>,
            Send<Fail, Recv<Received, End>>,
            RoleA<RoleS<RoleS<RoleEnd>>>,
            NameC,
        >,
    ),
}
type Offer0fromCtoA = Recv<Branching0fromAtoC, End>;

// S
enum Branching0fromAtoS {
    Success(
        MeshedChannelsThree<
            Send<Token, Recv<Token, End>>,
            Recv<Success, Send<Token, End>>,
            RoleC<RoleA<RoleA<RoleC<RoleEnd>>>>,
            NameS,
        >,
    ),
    Fail(MeshedChannelsThree<End, Recv<Fail, Send<Received, End>>, RoleC<RoleC<RoleEnd>>, NameS>),
}
type Offer0fromStoA = Recv<Branching0fromAtoS, End>;

// Creating the MP sessions
// A
type EndpointASuccess = MeshedChannelsThree<
    Send<Success, End>,
    Recv<Token, Send<Token, End>>,
    RoleC<RoleS<RoleS<RoleEnd>>>,
    NameA,
>;
type EndpointAFail = MeshedChannelsThree<Send<Fail, End>, End, RoleC<RoleEnd>, NameA>;
type EndpointA = MeshedChannelsThree<
    Recv<Login, Send<Auth, Recv<Password, Choose0fromAtoC>>>,
    Choose0fromAtoS,
    RoleC<RoleC<RoleC<RoleBroadcast>>>,
    NameA,
>;

// C
type EndpointC = MeshedChannelsThree<
    Send<Login, Recv<Auth, Send<Password, Offer0fromCtoA>>>,
    Send<Start, Recv<Redirect, End>>,
    RoleS<RoleS<RoleA<RoleA<RoleA<RoleA<RoleEnd>>>>>>,
    NameC,
>;

// S
type EndpointS = MeshedChannelsThree<
    Offer0fromStoA,
    Recv<Start, Send<Redirect, End>>,
    RoleC<RoleC<RoleA<RoleEnd>>>,
    NameS,
>;

// Functions
fn endpoint_a(s: EndpointA) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    let s = s.send(Auth {})?;
    let (_, s) = s.recv()?;

    let expected = thread_rng().gen_range(1..=3);

    if 1 == expected {
        let s: EndpointASuccess =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Success, Branching0fromAtoS::Success);

        let s = s.send(Success {})?;
        let (_, s) = s.recv()?;
        let s = s.send(Token {})?;
        s.close()
    } else {
        let s: EndpointAFail =
            choose_mpst_a_to_all!(s, Branching0fromAtoC::Fail, Branching0fromAtoS::Fail);

        let s = s.send(Fail {})?;
        s.close()
    }
}

fn endpoint_c(s: EndpointC) -> Result<(), Box<dyn Error>> {
    let s = s.send(Start {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Login {})?;
    let (_, s) = s.recv()?;
    let s = s.send(Password {})?;

    offer_mpst!(s, {
        Branching0fromAtoC::Success(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Success {  })?;
            let (_,s) = s.recv()?;
            s.close()
        },
        Branching0fromAtoC::Fail(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Fail {  })?;
            let (_, s) = s.recv()?;
            s.close()
        },
    })
}

fn endpoint_s(s: EndpointS) -> Result<(), Box<dyn Error>> {
    let (_, s) = s.recv()?;
    let s = s.send(Redirect {})?;

    offer_mpst!(s, {
        Branching0fromAtoS::Success(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Token {  })?;
            let (_, s) = s.recv()?;
            let s = s.send(Token {  })?;
            s.close()
        },
        Branching0fromAtoS::Fail(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Received {  })?;
            s.close()
        },
    })
}

// Check for bottom-up approach
fn checking() {
    let _ = checker_concat!(
        "o_auth_checking",
        EndpointA,
        EndpointC,
        EndpointS
        =>
        [
            EndpointASuccess,
            Branching0fromAtoC, Success,
            Branching0fromAtoS, Success,
        ],
        [
            EndpointAFail,
            Branching0fromAtoC, Fail,
            Branching0fromAtoS, Fail,
        ]
    )
    .unwrap();

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        std::fs::read_to_string("outputs/o_auth_checking_1_kmc.txt").unwrap()
    );
}

fn main() {
    checking();

    let (thread_a, thread_c, thread_s) = fork_mpst(endpoint_a, endpoint_c, endpoint_s);

    assert!(thread_a.join().is_ok());
    assert!(thread_c.join().is_ok());
    assert!(thread_s.join().is_ok());
}
