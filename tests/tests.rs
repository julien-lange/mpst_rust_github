extern crate mpst;

use mpst::*;
use std::boxed::Box;
use std::error::Error;

/// A = !B.?C
/// B = ?A.!C
/// C = !A.?B

type AtoB<N> = Send<N, End>;
type AtoC<N> = Recv<N, End>;

//type BtoA<N> = <AtoB<N> as Session>::Dual;
type BtoA<N> = Recv<N, End>;
type BtoC<N> = Send<N, End>;

//type CtoA<N> = <AtoC<N> as Session>::Dual;
//type CtoB<N> = <BtoC<N> as Session>::Dual;
type CtoA<N> = Send<N, End>;
type CtoB<N> = Recv<N, End>;

type EndpointA<N> = SessionMpst<AtoB<N>, AtoC<N>>;
type EndpointADual<N> = SessionMpst<<AtoB<N> as Session>::Dual, <AtoC<N> as Session>::Dual>;

type EndpointB<N> = SessionMpst<BtoA<N>, BtoC<N>>;

type EndpointC<N> = SessionMpst<CtoA<N>, CtoB<N>>;

fn endpoint_A(s: EndpointA<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_session_one(1, s);
    let (x, s) = recv_mpst_session_two(s)?;
    close_mpst(s)?;
    Ok(())
}

fn endpoint_A_for_B(s: AtoB<i32>) -> Result<(), Box<dyn Error>> {
    let s = send(1, s);
    close(s)?;
    Ok(())
}

fn endpoint_A_for_C(s: AtoC<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv(s)?;
    close(s)?;
    Ok(())
}

fn endpoint_B(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
    let (x, s) = recv_mpst_session_one(s)?;
    let s = send_mpst_session_two(x, s);
    close_mpst(s)?;
    Ok(())
}

fn endpoint_C(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
    let s = send_mpst_session_one(2, s);
    let (x, s) = recv_mpst_session_two(s)?;
    close_mpst(s)?;
    Ok(())
}

//fn endpoint_B(s: EndpointB<i32>) -> Result<(), Box<dyn Error>> {
//    let (x, s) = recv_mpst_session_one(s)?;
//    let s = send_mpst_session_two(x, s);
//    close_mpst_one(s)?;
//    close_mpst_two(s)?;
//    Ok(())
//}
//
//fn endpoint_C(s: EndpointC<i32>) -> Result<(), Box<dyn Error>> {
//    let s = send_mpst_session_one(2, s);
//    let (x, s) = recv_mpst_session_two(s)?;
//    close_mpst_one(s)?;
//    close_mpst_two(s)?;
//    Ok(())
//}

#[test]
fn simple_triple_endpoint() {
    assert!(|| -> Result<(), Box<dyn Error>> {

        // Test endpoint A 
        {
            let s: EndpointADual<i32> = fork_mpst(endpoint_A_for_B, endpoint_A_for_C);

            let (x, s) = recv_mpst_session_one(s)?;
            let s = send_mpst_session_two(1, s);
            close_mpst(s)?;

            assert_eq!(x, 1);
        }

        Ok(())

    }().is_ok());
}
