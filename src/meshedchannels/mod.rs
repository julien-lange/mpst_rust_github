#![cfg(feature = "meshedchannels")]

//! The main structure used for representing a participant,
//! also named a party, within a protocol.
//!
//! It contains 4 fields:
//! - **session1**: contains the first binary session type, which links the participant to the first
//!   participant in the alphanumerical order. It contains [`Session`].
//! - **session2**: contains the second binary session type, which links the participant to the
//!   second participant in the alphanumerical order. It contains [`Session`].
//! - **stack**: contains the ordering of the interactions between the participant and the others.
//!   It contains [`Role`].
//! - **name**: contains the name of the participant. It should look like `RoleA<RoleEnd>` or
//!   `RoleB<RoleEnd>`.
//!
//! [`Session`]: crate::binary::struct_trait::session::Session
//! [`Role`]: crate::role::Role

use crate::binary::struct_trait::session::Session;
use crate::role::Role;

pub mod impl_a;
pub mod impl_b;
pub mod impl_c;

/// The structure which encapsulates two binary session
/// types, a stack and a name.
///
/// # Arguments
///
/// * The first binary [`session`](crate::binary::struct_trait::session::Session). It must be filled
///   with [`Send`](crate::binary::struct_trait::send::Send) and/or
///   [`Recv`](crate::binary::struct_trait::recv::Recv) and end with
///   [`End`](crate::binary::struct_trait::end::End).
///
/// * The second binary [`session`](crate::binary::struct_trait::session::Session). It must be
///   filled with [`Send`](crate::binary::struct_trait::send::Send) and/or
///   [`Recv`](crate::binary::struct_trait::recv::Recv) and end with
///   [`End`](crate::binary::struct_trait::end::End).
///
/// * The stack of the MeshedChannels. It must be filled with a role, such as
///   [`RoleA`](crate::role::a::RoleA) or [`RoleBtoAll`](crate::role::b_to_all::RoleBtoAll) and end
///   with [`RoleEnd`](crate::role::end::RoleEnd).
///
/// * The name of the role of the MeshedChannels. It must be one among: *RoleA<RoleEnd>*,
///   *RoleB<RoleEnd>* or *RoleC<RoleEnd>*.
///
/// # Example
///
/// ```
/// use mpstthree::binary::struct_trait::{end::End, recv::Recv, send::Send};
///
/// use mpstthree::meshedchannels::MeshedChannels;
///
/// use mpstthree::binary::struct_trait::session::Session; // Only used for example
/// use mpstthree::role::a::RoleA;
/// use mpstthree::role::b::RoleB;
/// use mpstthree::role::c::RoleC;
/// use mpstthree::role::end::RoleEnd;
///
/// // Creating the binary sessions
/// type AtoB<N> = Send<N, End>;
/// type AtoC<N> = Recv<N, End>;
///
/// // Stacks
/// type StackA = RoleB<RoleC<RoleEnd>>;
///
/// // Creating the MP sessions
/// type EndpointA<N> = MeshedChannels<AtoB<N>, AtoC<N>, StackA, RoleA<RoleEnd>>;
///
/// let _ = EndpointA::<i32>::new(); // Only used for example
/// ```
#[must_use]
#[derive(Debug)]
pub struct MeshedChannels<S1, S2, R, N>
where
    S1: Session,
    S2: Session,
    R: Role,
    N: Role,
{
    #[doc(hidden)]
    pub session1: S1,
    #[doc(hidden)]
    pub session2: S2,
    #[doc(hidden)]
    pub stack: R,
    #[doc(hidden)]
    pub name: N,
}

#[doc(hidden)]
impl<S1: Session, S2: Session, R: Role, N: Role> Session for MeshedChannels<S1, S2, R, N> {
    type Dual = MeshedChannels<
        <S1 as Session>::Dual,
        <S2 as Session>::Dual,
        <R as Role>::Dual,
        <N as Role>::Dual,
    >;

    #[doc(hidden)]
    fn new() -> (Self, Self::Dual) {
        let (sender_one, receiver_one) = S1::new();
        let (sender_two, receiver_two) = S2::new();

        let (role_one, role_two) = R::new();
        let (name_one, name_two) = N::new();

        (
            MeshedChannels {
                session1: sender_one,
                session2: sender_two,
                stack: role_one,
                name: name_one,
            },
            MeshedChannels {
                session1: receiver_one,
                session2: receiver_two,
                stack: role_two,
                name: name_two,
            },
        )
    }

    #[doc(hidden)]
    fn head_str() -> String {
        format!(
            "{}\n{}\n{}\n{}",
            <S1 as Session>::head_str(),
            <S2 as Session>::head_str(),
            <R as Role>::head_str(),
            <N as Role>::head_str()
        )
    }

    #[doc(hidden)]
    fn tail_str() -> String {
        format!(
            "{}<{}>\n{}<{}>\n{}<{}>\n{}<{}>",
            <S1 as Session>::head_str(),
            <S1 as Session>::tail_str(),
            <S2 as Session>::head_str(),
            <S2 as Session>::tail_str(),
            <R as Role>::head_str(),
            <R as Role>::tail_str(),
            <N as Role>::head_str(),
            <N as Role>::tail_str(),
        )
    }

    #[doc(hidden)]
    fn self_head_str(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}",
            <S1 as Session>::head_str(),
            <S2 as Session>::head_str(),
            <R as Role>::head_str(),
            <N as Role>::head_str()
        )
    }

    #[doc(hidden)]
    fn self_tail_str(&self) -> String {
        format!(
            "{}<{}>\n{}<{}>\n{}<{}>\n{}<{}>",
            <S1 as Session>::head_str(),
            <S1 as Session>::tail_str(),
            <S2 as Session>::head_str(),
            <S2 as Session>::tail_str(),
            <R as Role>::head_str(),
            <R as Role>::tail_str(),
            <N as Role>::head_str(),
            <N as Role>::tail_str(),
        )
    }
}

#[doc(hidden)]
impl<S1: Session, S2: Session, R: Role, N: Role> MeshedChannels<S1, S2, R, N> {
    #[doc(hidden)]
    pub fn field_names(self) -> (&'static [&'static str], MeshedChannels<S1, S2, R, N>) {
        (&["session1", "session2"], self)
    }
}

impl<S1: Session, S2: Session, R: Role, N: Role> MeshedChannels<S1, S2, R, N> {
    /// Cancel the session
    pub fn cancel(self) {
        std::mem::drop(self);
    }
}
