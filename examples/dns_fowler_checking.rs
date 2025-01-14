#![allow(clippy::type_complexity)]

use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
use mpstthree::role::broadcast::RoleBroadcast;
use mpstthree::role::end::RoleEnd;
use mpstthree::{bundle_impl_with_enum_and_cancel, checker_concat};

use rand::{thread_rng, Rng};

use std::error::Error;

// See the folder scribble_protocols for the related Scribble protocol

// Create the new MeshedChannels for three participants and the close and fork functions
bundle_impl_with_enum_and_cancel!(MeshedChannelsThree, Data, Handler, Regional);

// Payload types
struct FindNearestZone;
struct ZoneResponse;
struct ZoneDataRequest;
struct ZoneDataResponse;
struct InvalidZone;
struct Received;

// Names
type NameData = RoleData<RoleEnd>;
type NameHandler = RoleHandler<RoleEnd>;
type NameRegional = RoleRegional<RoleEnd>;

// Types
// REGIONAL
type Choose0fromRegionalToData = Send<Branching0fromRegionalToData, End>;
type Choose0fromRegionalToHandler = Send<Branching0fromRegionalToHandler, End>;
// DATA
enum Branching0fromRegionalToData {
    Loops(
        MeshedChannelsThree<
            Recv<ZoneDataRequest, Send<ZoneDataResponse, End>>,
            Offer0fromRegionalToData,
            RoleHandler<RoleHandler<RoleRegional<RoleEnd>>>,
            NameData,
        >,
    ),
    Invalid(
        MeshedChannelsThree<
            Recv<InvalidZone, Send<Received, End>>,
            End,
            RoleHandler<RoleHandler<RoleEnd>>,
            NameData,
        >,
    ),
}
type Offer0fromRegionalToData = Recv<Branching0fromRegionalToData, End>;
// HANDLER
enum Branching0fromRegionalToHandler {
    Loops(
        MeshedChannelsThree<
            Send<ZoneDataRequest, Recv<ZoneDataResponse, End>>,
            Recv<ZoneResponse, Send<FindNearestZone, Offer0fromRegionalToHandler>>,
            RoleRegional<RoleData<RoleData<RoleRegional<RoleRegional<RoleEnd>>>>>,
            NameHandler,
        >,
    ),
    Invalid(
        MeshedChannelsThree<
            Send<InvalidZone, Recv<Received, End>>,
            Recv<InvalidZone, End>,
            RoleRegional<RoleData<RoleData<RoleEnd>>>,
            NameHandler,
        >,
    ),
}
type Offer0fromRegionalToHandler = Recv<Branching0fromRegionalToHandler, End>;

// Creating the MP sessions
// DATA
type EndpointData =
    MeshedChannelsThree<End, Offer0fromRegionalToData, RoleRegional<RoleEnd>, NameData>;
// HANDLER
type EndpointHandler = MeshedChannelsThree<
    End,
    Send<FindNearestZone, Offer0fromRegionalToHandler>,
    RoleRegional<RoleRegional<RoleEnd>>,
    NameHandler,
>;
// REGIONAL
type EndpointRegionalInvalid =
    MeshedChannelsThree<End, Send<InvalidZone, End>, RoleHandler<RoleEnd>, NameRegional>;
type EndpointRegionalLoops = MeshedChannelsThree<
    Choose0fromRegionalToData,
    Send<ZoneResponse, Recv<FindNearestZone, Choose0fromRegionalToHandler>>,
    RoleHandler<RoleHandler<RoleBroadcast>>,
    NameRegional,
>;
type EndpointRegional = MeshedChannelsThree<
    Choose0fromRegionalToData,
    Recv<FindNearestZone, Choose0fromRegionalToHandler>,
    RoleHandler<RoleBroadcast>,
    NameRegional,
>;

// Functions
fn endpoint_data(s: EndpointData) -> Result<(), Box<dyn Error>> {
    offer_mpst!(s, {
        Branching0fromRegionalToData::Loops(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(ZoneDataResponse {})?;
            endpoint_data(s)
        },
        Branching0fromRegionalToData::Invalid(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(Received {})?;
            s.close()
        },
    })
}

fn endpoint_handler(s: EndpointHandler) -> Result<(), Box<dyn Error>> {
    let s = s.send(FindNearestZone {})?;

    offer_mpst!(s, {
        Branching0fromRegionalToHandler::Loops(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(ZoneDataRequest {} )?;
            let (_, s) = s.recv()?;
            endpoint_handler(s)
        },
        Branching0fromRegionalToHandler::Invalid(s) => {
            let (_, s) = s.recv()?;
            let s = s.send(InvalidZone {} )?;
            let (_, s) = s.recv()?;
            s.close()
        },
    })
}

fn endpoint_regional(s: EndpointRegional) -> Result<(), Box<dyn Error>> {
    let choice = thread_rng().gen_range(1..3);

    let (_, s) = s.recv()?;

    if choice == 1 {
        let s: EndpointRegionalLoops = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Loops,
            Branching0fromRegionalToHandler::Loops
        );
        let s = s.send(ZoneResponse {})?;
        endpoint_regional(s)
    } else {
        let s: EndpointRegionalInvalid = choose_mpst_regional_to_all!(
            s,
            Branching0fromRegionalToData::Invalid,
            Branching0fromRegionalToHandler::Invalid
        );
        let s = s.send(InvalidZone {})?;
        s.close()
    }
}

// Check for bottom-up approach
fn checking() {
    let _ = checker_concat!(
        "dns_fowler",
        EndpointHandler,
        EndpointData,
        EndpointRegional
        =>
        [
            EndpointRegionalLoops,
            Branching0fromRegionalToData, Loops,
            Branching0fromRegionalToHandler, Loops,
        ],
        [
            EndpointRegionalInvalid,
            Branching0fromRegionalToData, Invalid,
            Branching0fromRegionalToHandler, Invalid,
        ]
    )
    .unwrap();

    assert_eq!(
        "CSA: \u{1b}[92mTrue\n\
        \u{1b}[0mBasic: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-exhaustive: \u{1b}[92mTrue\n\
        \u{1b}[0mreduced 1-safe: \u{1b}[92mTrue\n\
        \u{1b}[0m\n",
        std::fs::read_to_string("outputs/dns_fowler_1_kmc.txt").unwrap()
    );
}

fn main() {
    checking();

    let (thread_handler, thread_regional, thread_data) =
        fork_mpst(endpoint_data, endpoint_handler, endpoint_regional);

    assert!(thread_data.join().is_ok());
    assert!(thread_regional.join().is_ok());
    assert!(thread_handler.join().is_ok());
}
