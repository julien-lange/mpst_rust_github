use proc_macro2::{Span, TokenStream, TokenTree};
use quote::quote;
use std::convert::TryFrom;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

type VecOfTuple = Vec<(u64, u64, u64)>;

#[derive(Debug)]
pub struct BakingWithCancel {
    meshedchannels_name: Ident,
    all_roles: Vec<TokenStream>,
    number_roles: u64,
}

fn expand_token_stream(input: ParseStream) -> Result<Vec<TokenStream>> {
    let content;
    let _parentheses = syn::parenthesized!(content in input);
    let token_stream = TokenStream::parse(&content)?;

    let mut result: Vec<TokenStream> = Vec::new();
    for tt in token_stream.into_iter() {
        let elt = match tt {
            TokenTree::Group(g) => Some(g.stream()),
            _ => None,
        };
        if let Some(elt_tt) = elt {
            result.push(elt_tt)
        }
    }

    Ok(result)
}

impl Parse for BakingWithCancel {
    fn parse(input: ParseStream) -> Result<Self> {
        let meshedchannels_name = Ident::parse(input)?;
        <Token![,]>::parse(input)?;
        let all_roles = expand_token_stream(<&syn::parse::ParseBuffer>::clone(&input))?;

        let number_roles = u64::try_from(all_roles.len()).unwrap();

        Ok(BakingWithCancel {
            meshedchannels_name,
            all_roles,
            number_roles,
        })
    }
}

impl From<BakingWithCancel> for TokenStream {
    fn from(input: BakingWithCancel) -> TokenStream {
        input.expand()
    }
}

impl BakingWithCancel {
    /// Create the whole matrix of index according to line and column
    fn diag(&self) -> VecOfTuple {
        let diff = self.number_roles - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.number_roles - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    /// Create the whole matrix of index according to line and column
    fn diag_and_matrix(&self) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag = self.diag();

        // Create the whole matrix
        (
            self.diag(),
            (1..=self.number_roles)
                .map(|i| {
                    diag.iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                Some((*line, *column, *index))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Create the whole matrix of index according to line and column
    fn diag_w_offset(&self) -> VecOfTuple {
        let diff = self.number_roles - 1;

        let mut column = 0;
        let mut line = 0;

        // Create the upper diag
        (0..=(diff * (diff + 1) / 2))
            .map(|i| {
                if line == column {
                    column += 1;
                } else if column >= (self.number_roles - 1) {
                    line += 1;
                    column = line + 1;
                } else {
                    column += 1;
                }
                (line + 1, column + 1, i + 1)
            })
            .collect()
    }

    /// Create the whole matrix of index according to line and column
    fn diag_and_matrix_w_offset(&self) -> (VecOfTuple, Vec<VecOfTuple>) {
        let diag_w_offset = self.diag_w_offset();

        // Create the whole matrix
        (
            diag_w_offset.clone(),
            (1..=self.number_roles)
                .map(|i| {
                    diag_w_offset
                        .iter()
                        .filter_map(|(line, column, index)| {
                            if i == *line || i == *column {
                                Some((*line, *column, *index))
                            } else {
                                None
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    /// Return (line, column, index) of diag
    fn get_tuple_diag(&self, diag: &[(u64, u64, u64)], i: u64) -> (u64, u64, u64) {
        if let Some((line, column, index)) = diag.get(usize::try_from(i - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            panic!(
                "Error at get_tuple_diag for i = {:?} / diag = {:?}",
                i, diag
            )
        }
    }

    /// Return (line, column, index) of matrix
    fn get_tuple_matrix(&self, matrix: &[VecOfTuple], i: u64, j: u64) -> (u64, u64, u64) {
        let list: VecOfTuple = if let Some(temp) = matrix.get(usize::try_from(i - 1).unwrap()) {
            temp.to_vec()
        } else {
            panic!(
                "Error at get_tuple_matrix for i = {:?} / matrix = {:?}",
                i, matrix
            )
        };

        if let Some((line, column, index)) = list.get(usize::try_from(j - 1).unwrap()) {
            (*line, *column, *index)
        } else {
            panic!("Error at get_tuple_matrix for i = {:?} and j = {:?} with list = {:?} / matrix = {:?}", i, j, list, matrix)
        }
    }

    /// Return (line, column) of diag from index
    fn get_line_column_from_diag(&self, diag: &[(u64, u64, u64)], index: u64) -> (u64, u64) {
        for i in diag {
            if i.2 == index {
                return (i.0, i.1);
            }
        }
        panic!("Error at get_line_column_from_diag for index = {:?}", index)
    }

    /// Expand send methods
    fn expand_send(
        &self,
        all_roles: Vec<TokenStream>,
        sender: u64,
        receiver: u64,
        session_types: Vec<Ident>,
        session_types_struct: Vec<TokenStream>,
    ) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("Role{}", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_send")
        };

        let receiver_ident =
            if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
                Ident::new(&format!("Role{}", elt), Span::call_site())
            } else {
                panic!("Not enough arguments for receiver_ident in expand_send")
            };

        let send_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= sender { receiver - 1 } else { receiver };

                let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

                if k == cond {
                    quote! { mpstthree::binary::struct_trait::send::Send<T, #temp_type > ,}
                } else {
                    quote! { #temp_type , }
                }
            })
            .collect();

        let new_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= sender { receiver - 1 } else { receiver };

                let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

                if k == cond {
                    quote! { #temp_session : new_session , }
                } else {
                    quote! { #temp_session : self.#temp_session , }
                }
            })
            .collect();

        let index = if receiver >= sender {
            receiver - 1
        } else {
            receiver
        };

        let new_session = Ident::new(&format!("session{}", index), Span::call_site());

        quote! {
            impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
                #meshedchannels_name<
                    #( #send_sessions )*
                    #receiver_ident<R>,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn send(self, payload: T) -> Result<
                    #meshedchannels_name<
                        #( #session_types , )*
                        R,
                        #sender_ident<mpstthree::role::end::RoleEnd>
                    >,
                    Box<dyn std::error::Error>
                > {
                    let new_session = mpstthree::binary::send::send_canceled(payload, self.#new_session)?;
                    let new_stack = self.stack.continuation();
                    Ok(
                        #meshedchannels_name {
                            #( #new_sessions )*
                            stack: new_stack,
                            name: self.name,
                        }
                    )
                }
            }
        }
    }

    /// Expand receive methods
    fn expand_recv(
        &self,
        all_roles: Vec<TokenStream>,
        receiver: u64,
        sender: u64,
        session_types: Vec<Ident>,
        session_types_struct: Vec<TokenStream>,
    ) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("Role{}", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_recv")
        };

        let receiver_ident =
            if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
                Ident::new(&format!("Role{}", elt), Span::call_site())
            } else {
                panic!("Not enough arguments for receiver_ident in expand_recv")
            };

        let send_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };

                let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

                if k == cond {
                    quote! { mpstthree::binary::struct_trait::recv::Recv<T, #temp_type > ,}
                } else {
                    quote! { #temp_type ,}
                }
            })
            .collect();

        let new_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };

                let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

                if k == cond {
                    quote! { #temp_session : new_session , }
                } else {
                    quote! { #temp_session : self.#temp_session , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = Ident::new(&format!("session{}", index), Span::call_site());

        quote! {
            impl<#( #session_types_struct )* R: mpstthree::role::Role, T: std::marker::Send>
                #meshedchannels_name<
                    #( #send_sessions )*
                    #sender_ident<R>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv(self) -> Result<(
                    T,
                    #meshedchannels_name<
                        #( #session_types , )*
                        R,
                        #receiver_ident<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    let (v, new_session) = mpstthree::binary::recv::recv(self.#new_session)?;
                    let new_stack = self.stack.continuation();
                    Ok((
                        v,
                        #meshedchannels_name {
                            #( #new_sessions )*
                            stack: new_stack,
                            name: self.name,
                        }
                    ))
                }
            }
        }
    }

    /// Expand receive from all methods
    fn expand_recv_from_all(
        &self,
        all_roles: Vec<TokenStream>,
        receiver: u64,
        sender: u64,
        session_types: Vec<Ident>,
        session_types_struct: Vec<TokenStream>,
    ) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("RoleAllto{}", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_recv_from_all")
        };

        let receiver_ident =
            if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
                Ident::new(&format!("Role{}", elt), Span::call_site())
            } else {
                panic!("Not enough arguments for receiver_ident in expand_recv_from_all")
            };

        let send_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };

                let temp_type = Ident::new(&format!("S{}", k), Span::call_site());

                if k == cond {
                    quote! { mpstthree::binary::struct_trait::recv::Recv<T, #temp_type > ,}
                } else {
                    quote! { #temp_type ,}
                }
            })
            .collect();

        let new_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };

                let temp_session = Ident::new(&format!("session{}", k), Span::call_site());

                if k == cond {
                    quote! { #temp_session : new_session , }
                } else {
                    quote! { #temp_session : self.#temp_session , }
                }
            })
            .collect();

        let index = if sender >= receiver {
            sender - 1
        } else {
            sender
        };

        let new_session = Ident::new(&format!("session{}", index), Span::call_site());

        quote! {
            impl<#( #session_types_struct )* T: std::marker::Send>
                #meshedchannels_name<
                    #( #send_sessions )*
                    #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn recv_from_all(self) -> Result<(
                    T,
                    #meshedchannels_name<
                        #( #session_types , )*
                        mpstthree::role::end::RoleEnd,
                        #receiver_ident<mpstthree::role::end::RoleEnd>
                    >),
                    Box<dyn std::error::Error>
                > {
                    let (v, new_session) = mpstthree::binary::recv::recv(self.#new_session)?;

                    let new_stack = self.stack.continuation_left();

                    Ok((
                        v,
                        #meshedchannels_name {
                            #( #new_sessions )*
                            stack: new_stack,
                            name: self.name,
                        }
                    ))
                }
            }
        }
    }

    /// Expand offer methods
    fn expand_offer(&self, all_roles: Vec<TokenStream>, sender: u64, receiver: u64) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("RoleAllto{}", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_offer")
        };

        let receiver_ident =
            if let Some(elt) = all_roles.get(usize::try_from(receiver - 1).unwrap()) {
                Ident::new(&format!("Role{}", elt), Span::call_site())
            } else {
                panic!("Not enough arguments for receiver_ident in expand_offer")
            };

        let offer_session_types_struct: Vec<TokenStream> = (1..(2 * self.number_roles - 1))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let left_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident , }
            })
            .collect();

        let right_sessions: Vec<TokenStream> = (self.number_roles..(2 * self.number_roles - 1))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident , }
            })
            .collect();

        let offer_sessions: Vec<TokenStream> = (1..self.number_roles)
            .map(|k| {
                let cond = if k >= receiver { sender - 1 } else { sender };
                if k == cond {
                    quote! {
                        mpstthree::binary::struct_trait::recv::Recv<
                            either::Either<
                                #meshedchannels_name<
                                    #( #left_sessions )*
                                    R1,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >,
                                #meshedchannels_name<
                                    #( #right_sessions )*
                                    R2,
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >
                            >,
                            mpstthree::binary::struct_trait::end::End
                        >,
                    }
                } else {
                    quote! { mpstthree::binary::struct_trait::end::End, }
                }
            })
            .collect();

        quote! {
            impl<
                'a,
                #( #offer_session_types_struct )*
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
            >
                #meshedchannels_name<
                    #( #offer_sessions )*
                    #sender_ident<mpstthree::role::end::RoleEnd, mpstthree::role::end::RoleEnd>,
                    #receiver_ident<mpstthree::role::end::RoleEnd>,
                >
            {
                pub fn offer<F, G, U>(self, f: F, g: G) -> Result<U, Box<dyn std::error::Error + 'a>>
                where
                    F: FnOnce(
                        #meshedchannels_name<
                            #( #left_sessions )*
                            R1,
                            #receiver_ident<mpstthree::role::end::RoleEnd>,
                        >,
                    ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                    G: FnOnce(
                        #meshedchannels_name<
                            #( #right_sessions )*
                            R2,
                            #receiver_ident<mpstthree::role::end::RoleEnd>,
                        >,
                    ) -> Result<U, Box<dyn std::error::Error + 'a>>,
                {
                    let (e, s) = self.recv_from_all()?;
                    mpstthree::binary::cancel::cancel(s);
                    e.either(f, g)
                }
            }
        }
    }

    /// Expand choose methods
    fn expand_choose(&self, all_roles: Vec<TokenStream>, sender: u64) -> TokenStream {
        let (diag, matrix) = self.diag_and_matrix();
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("Role{}", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_choose")
        };

        let sender_stack = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            Ident::new(&format!("Role{}toAll", elt), Span::call_site())
        } else {
            panic!("Not enough arguments for sender_stack in expand_choose")
        };

        let choose_session_types_struct: Vec<TokenStream> = (1..=((self.number_roles - 1)
            * self.number_roles))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let choose_roles_struct: Vec<TokenStream> = (1..=(2 * self.number_roles))
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::role::Role , }
            })
            .collect();

        let choose_sessions: Vec<TokenStream> = (1..=self.number_roles)
            .map(|j| {
                if sender != j {
                    let left_sessions: Vec<TokenStream> = (1..self.number_roles)
                        .map(|k| {

                            let (l, _, _) = self.get_tuple_matrix(&matrix, j, k);

                            if l == 0 {
                                panic!("Erratum choose_sessions / left_sessions j = {:?}", j)
                            };

                            let (_, _, m1) = if j > sender {
                                self.get_tuple_matrix(&matrix, sender, j - 1)
                            } else {
                                self.get_tuple_matrix(&matrix, sender, j)
                            };
                            let (_, _, m2) = self.get_tuple_matrix(&matrix, j, k);

                            let (_, _, m) = self.get_tuple_matrix(&matrix, j, k);

                            let temp_ident = Ident::new(
                                &format!("S{}", m),
                                Span::call_site(),
                            );

                            if l == j || m1 == m2 {
                                quote! { #temp_ident , }
                            } else {
                                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
                            }
                        })
                        .collect();

                    let right_sessions: Vec<TokenStream> = (1..self.number_roles)
                        .map(|k| {

                            let (l, _, _) = self.get_tuple_matrix(&matrix, j, k);

                            if l == 0 {
                                panic!("Erratum choose_sessions / right_sessions j = {:?}", j)
                            };

                            let (_, _, m1) = if j > sender {
                                self.get_tuple_matrix(&matrix, sender, j - 1)
                            } else {
                                self.get_tuple_matrix(&matrix, sender, j)
                            };
                            let (_, _, m2) = self.get_tuple_matrix(&matrix, j, k);

                            let (_, _, m) = self.get_tuple_matrix(&matrix, j, k);

                            let diff = self.number_roles - 1;

                            let temp_ident = Ident::new(
                                &format!("S{}", diff * (diff + 1) / 2 + m),
                                Span::call_site(),
                            );

                            if l == j || m1 == m2 {
                                quote! { #temp_ident , }
                            } else {
                                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
                            }
                        })
                        .collect();

                    let stack_left = if j > sender {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) - 1),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    } else {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) + 1),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    };

                    let stack_right = if j > sender {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1)),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    } else {
                        let temp_ident = Ident::new(
                            &format!("R{}", 2 * (j - 1) + 2),
                            Span::call_site(),
                        );
                        quote! { #temp_ident , }
                    };

                    let receiver_ident =
                        if let Some(elt) = all_roles.get(usize::try_from(j - 1).unwrap()) {
                            Ident::new(&format!("Role{}", elt), Span::call_site())
                        } else {
                            panic!("Not enough arguments for receiver_ident in choose_sessions in expand_choose")
                        };

                    quote! {
                        mpstthree::binary::struct_trait::send::Send<
                            either::Either<
                                #meshedchannels_name<
                                    #(
                                        #left_sessions
                                    )*
                                    #stack_left
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >,
                                #meshedchannels_name<
                                    #(
                                        #right_sessions
                                    )*
                                    #stack_right
                                    #receiver_ident<mpstthree::role::end::RoleEnd>
                                >
                            >,
                            mpstthree::binary::struct_trait::end::End,
                        >,
                    }
                } else {
                    quote! {
                        // Empty
                    }
                }
            })
            .collect();

        let new_stack_sender_left = Ident::new(
            &format!("R{}", 2 * self.number_roles - 1),
            Span::call_site(),
        );
        let new_stack_sender_right =
            Ident::new(&format!("R{}", 2 * self.number_roles), Span::call_site());
        let new_stacks_sender = quote! { #new_stack_sender_left , #new_stack_sender_right };

        let choose_left_session: Vec<TokenStream> = (1..=self.number_roles)
            .filter_map(|j| {
                if j == sender {
                    None
                } else {
                    let (_, _, m) = if j > sender {
                        self.get_tuple_matrix(&matrix, sender, j - 1)
                    } else {
                        self.get_tuple_matrix(&matrix, sender, j)
                    };
                    let temp_ident =
                        Ident::new(&format!("S{}", m), Span::call_site());
                    Some(
                        quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual, },
                    )
                }
            })
            .collect();

        let choose_right_session: Vec<TokenStream> = (1..=self.number_roles)
            .filter_map(|j| {
                if j == sender {
                    None
                } else {
                    let (_, _, m) = if j > sender {
                        self.get_tuple_matrix(&matrix, sender, j - 1)
                    } else {
                        self.get_tuple_matrix(&matrix, sender, j)
                    };
                    let diff = self.number_roles - 1;
                    let temp_ident = Ident::new(
                        &format!("S{}", diff * (diff + 1) / 2 + m),
                        Span::call_site(),
                    );
                    Some(
                        quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual, },
                    )
                }
            })
            .collect();
        let choose_left_channels: Vec<TokenStream> =
            (1..=((self.number_roles - 1) * self.number_roles / 2))
                .map(|j| {
                    let (line, column) = self.get_line_column_from_diag(&diag, j);

                    let first_channel = if sender != line {
                        Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
                    } else {
                        Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
                    };

                    let second_channel = if sender != line {
                        Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
                    } else {
                        Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
                    };

                    let temp_session = Ident::new(&format!("S{}", j), Span::call_site());

                    quote! { let ( #first_channel , #second_channel ) =
                    <#temp_session as mpstthree::binary::struct_trait::session::Session>::new() ; }
                })
                .collect();

        let choose_right_channels: Vec<TokenStream> =
            (1..=((self.number_roles - 1) * self.number_roles / 2))
                .map(|j| {
                    let (line, column) = self.get_line_column_from_diag(&diag, j);
                    let diff = self.number_roles - 1;

                    let first_channel = if sender != line {
                        Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
                    } else {
                        Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
                    };

                    let second_channel = if sender != line {
                        Ident::new(&format!("channel_{}_{}", column, line), Span::call_site())
                    } else {
                        Ident::new(&format!("channel_{}_{}", line, column), Span::call_site())
                    };

                    let temp_session = Ident::new(
                        &format!("S{}", diff * (diff + 1) / 2 + j),
                        Span::call_site(),
                    );

                    quote! { let ( #first_channel , #second_channel ) = #temp_session::new() ; }
                })
                .collect();

        let new_stacks_receivers_left: Vec<TokenStream> = (1..self.number_roles)
            .map(|j| {
                let temp_stack = Ident::new(&format!("stack_{}", j), Span::call_site());
                let temp_role = Ident::new(&format!("R{}", 2 * (j - 1) + 1), Span::call_site());
                quote! { let (#temp_stack, _) = <#temp_role as mpstthree::role::Role>::new(); }
            })
            .collect();

        let new_stacks_receivers_right: Vec<TokenStream> = (1..self.number_roles)
            .map(|j| {
                let temp_stack = Ident::new(&format!("stack_{}", j), Span::call_site());
                let temp_role = Ident::new(&format!("R{}", 2 * (j - 1) + 2), Span::call_site());
                quote! { let (#temp_stack, _) = <#temp_role as mpstthree::role::Role>::new(); }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=self.number_roles)
        .map(|j| {
            if sender != j {

                let receiver_ident =
                    if let Some(elt) = all_roles.get(usize::try_from(j-1).unwrap()) {
                        Ident::new(&format!("Role{}", elt), Span::call_site())
                    } else {
                        panic!("Not enough arguments for receiver_ident in new_names in expand_choose")
                    };

                    let new_name =
                        if let Some(elt) = all_roles.get(usize::try_from(j-1).unwrap()) {
                            Ident::new(&format!("name_{}", elt), Span::call_site())
                        } else {
                            panic!("Not enough arguments for new_name in new_names in expand_choose")
                        };

                quote! {
                    let (#new_name, _) = <#receiver_ident<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();
                }
            } else {
                quote! { }
            }
        })
        .collect();

        let new_meshedchannels_receivers: Vec<TokenStream> = (1..=self.number_roles)
            .map(|j| {
                if sender != j {
                    let new_sessions_receiver: Vec<TokenStream> = (1..self
                        .number_roles)
                        .map(|k| {
                            let new_session_receiver = Ident::new(
                                &format!("session{}", k),
                                Span::call_site(),
                            );
                            let new_channel_receiver = if j > k {
                                Ident::new(
                                    &format!("channel_{}_{}", j, k),
                                    Span::call_site(),
                                )
                            } else {
                                Ident::new(
                                    &format!("channel_{}_{}", j, k + 1),
                                    Span::call_site(),
                                )
                            };

                            quote! { #new_session_receiver : #new_channel_receiver , }
                        })
                        .collect();

                    let new_choice_receiver = if j > sender
                    {
                        Ident::new(&format!("choice_{}", j - 1), Span::call_site())
                    } else {
                        Ident::new(&format!("choice_{}", j), Span::call_site())
                    };

                    let new_stack_receiver = if j > sender
                    {
                        Ident::new(&format!("stack_{}", j - 1), Span::call_site())
                    } else {
                        Ident::new(&format!("stack_{}", j), Span::call_site())
                    };

                    let new_name_receiver = if let Some(elt) =
                        all_roles.get(usize::try_from(j - 1).unwrap())
                    {
                        Ident::new(&format!("name_{}", elt), Span::call_site())
                    } else {
                        panic!("Not enough arguments for new_name_receiver in new_meshedchannels_receivers in expand_choose")
                    };

                    quote! {
                        let #new_choice_receiver = #meshedchannels_name {
                            #(
                                #new_sessions_receiver
                            )*
                            stack: #new_stack_receiver,
                            name: #new_name_receiver,
                        };
                    }
                } else {
                    quote! {
                        // Empty
                    }
                }
            })
            .collect();

        let new_sessions_sender_left: Vec<TokenStream> = (1..self.number_roles)
            .map(|j| {
                let new_session_sender =
                    Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

                let new_choice_sender = Ident::new(&format!("choice_{}", j), Span::call_site());

                let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

                quote! {
                    let #new_session_sender = mpstthree::binary::send::send(
                        either::Either::Left(#new_choice_sender),
                        self.#session_sender
                    );
                }
            })
            .collect();

        let new_sessions_sender_right: Vec<TokenStream> = (1..self.number_roles)
            .map(|j| {
                let new_session_sender =
                    Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

                let new_choice_sender = Ident::new(&format!("choice_{}", j), Span::call_site());

                let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

                quote! {
                    let #new_session_sender = mpstthree::binary::send::send(
                        either::Either::Right(#new_choice_sender),
                        self.#session_sender
                    );
                }
            })
            .collect();

        let old_meshedchannels_sender: Vec<TokenStream> = (1..self.number_roles)
            .map(|j| {
                let new_session_sender =
                    Ident::new(&format!("new_session_{}", j - 1), Span::call_site());

                let session_sender = Ident::new(&format!("session{}", j), Span::call_site());

                quote! {
                    #session_sender : #new_session_sender ,
                }
            })
            .collect();

        let new_meshedchannels_sender: Vec<TokenStream> = (1..=self.number_roles)
            .map(|j| {
                if sender != j {
                    let new_choice_sender = if j < sender {
                        Ident::new(&format!("session{}", j), Span::call_site())
                    } else {
                        Ident::new(&format!("session{}", j - 1), Span::call_site())
                    };

                    let new_channel_sender =
                        Ident::new(&format!("channel_{}_{}", sender, j), Span::call_site());

                    quote! {
                        #new_choice_sender : #new_channel_sender,
                    }
                } else {
                    quote! {
                        // Empty
                    }
                }
            })
            .collect();

        let new_stack_sender =
            Ident::new(&format!("stack_{}", self.number_roles), Span::call_site());

        let new_name_sender = Ident::new(&format!("name_{}", self.number_roles), Span::call_site());

        quote! {
            impl<
                #(
                    #choose_session_types_struct
                )*
                #(
                    #choose_roles_struct
                )*
            >
                #meshedchannels_name<
                    #(
                        #choose_sessions
                    )*
                    #sender_stack<
                        #new_stacks_sender
                    >,
                    #sender_ident<mpstthree::role::end::RoleEnd>,
                >
            {
                pub fn choose_left(self) -> #meshedchannels_name<
                    #(
                        #choose_left_session
                    )*
                    #new_stack_sender_left ,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                >
                {
                    #(
                        #choose_left_channels
                    )*

                    #(
                        #new_stacks_receivers_left
                    )*

                    let (#new_stack_sender, _) = <#new_stack_sender_left as mpstthree::role::Role>::new();

                    #(
                        #new_names
                    )*

                    let (#new_name_sender, _) = <#sender_ident::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                    #(
                        #new_meshedchannels_receivers
                    )*

                    #(
                        #new_sessions_sender_left
                    )*

                    let s = #meshedchannels_name {
                        #(
                            #old_meshedchannels_sender
                        )*
                        stack: self.stack,
                        name: self.name,
                    };

                    mpstthree::binary::cancel::cancel(s);

                    #meshedchannels_name {
                        #(
                            #new_meshedchannels_sender
                        )*
                        stack: #new_stack_sender,
                        name: #new_name_sender,
                    }
                }
                pub fn choose_right(self) -> #meshedchannels_name<
                    #(
                        #choose_right_session
                    )*
                    #new_stack_sender_right ,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                >
                {
                    #(
                        #choose_right_channels
                    )*

                    #(
                        #new_stacks_receivers_right
                    )*

                    let (#new_stack_sender, _) = <#new_stack_sender_right as mpstthree::role::Role>::new();

                    #(
                        #new_names
                    )*

                    let (#new_name_sender, _) = <#sender_ident::<mpstthree::role::end::RoleEnd> as mpstthree::role::Role>::new();

                    #(
                        #new_meshedchannels_receivers
                    )*

                    #(
                        #new_sessions_sender_right
                    )*

                    let s = #meshedchannels_name {
                        #(
                            #old_meshedchannels_sender
                        )*
                        stack: self.stack,
                        name: self.name,
                    };

                    mpstthree::binary::cancel::cancel(s);

                    #meshedchannels_name {
                        #(
                            #new_meshedchannels_sender
                        )*
                        stack: #new_stack_sender,
                        name: #new_name_sender,
                    }
                }
            }
        }
    }

    fn expand_close(&self, all_roles: Vec<TokenStream>, sender: u64) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let sender_ident = if let Some(elt) = all_roles.get(usize::try_from(sender - 1).unwrap()) {
            let concatenated_elt = format!("Role{}", elt);
            Ident::new(&concatenated_elt, Span::call_site())
        } else {
            panic!("Not enough arguments for sender_ident in expand_close")
        };

        let close_session_types: Vec<TokenStream> = (1..self.number_roles)
            .map(|_i| {
                quote! { mpstthree::binary::struct_trait::end::End, }
            })
            .collect();

        let close_session_send: Vec<TokenStream> = (1..self.number_roles)
                .map(|i| {
                    let temp_session = Ident::new(
                        &format!("session{}", i),
                        Span::call_site(),
                    );
                    quote! { self.#temp_session.sender.send(mpstthree::binary::struct_trait::end::Signal::Stop).unwrap_or(()); }
                })
                .collect();

        let close_session_recv: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                quote! { self.#temp_session.receiver.recv()?; }
            })
            .collect();

        quote! {
            impl
                #meshedchannels_name<
                    #(
                        #close_session_types
                    )*
                    mpstthree::role::end::RoleEnd,
                    #sender_ident<mpstthree::role::end::RoleEnd>
                >
            {
                pub fn close(self) -> Result<(), Box<dyn std::error::Error>> {

                    #(
                        #close_session_send
                    )*

                    #(
                        #close_session_recv
                    )*

                    Ok(())
                }
            }
        }
    }

    fn expand_cancel(&self) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        let temp_types: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_session , }
            })
            .collect();

        let temp_detail_types: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_session : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        quote! {
            impl<
                #(
                    #temp_detail_types
                )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role,
            >
                #meshedchannels_name<
                    #(
                        #temp_types
                    )*
                    R,
                    N
                >
            {
                /// Cancel the session
                pub fn cancel(self) {
                    std::mem::drop(self);
                }
            }
        }
    }

    fn expand_role(&self, role: String) -> TokenStream {
        // role
        let role_name = Ident::new(&format!("Role{}", role), Span::call_site());
        // dual
        let dual_name = Ident::new(&format!("Role{}Dual", role), Span::call_site());
        // role to all
        let role_to_all_name = Ident::new(&format!("Role{}toAll", role), Span::call_site());
        // dual to all
        let dual_to_all_name = Ident::new(&format!("RoleAllto{}", role), Span::call_site());

        quote! {
            ////////////////////////////////////////////
            /// The normal Role
            #[derive(Debug)]
            struct #role_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }
            ////////////////////////////////////////////
            /// The normal Dual
            #[derive(Debug)]
            struct #dual_name<R>
            where
                R: mpstthree::role::Role,
                R::Dual: mpstthree::role::Role,
            {
                sender: crossbeam_channel::Sender<R::Dual>,
            }
            ////////////////////////////////////////////
            /// The normal Role implementation of Role
            impl<R: mpstthree::role::Role> mpstthree::role::Role for #role_name<R> {
                type Dual = #dual_name<<R as mpstthree::role::Role>::Dual>;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
                    (
                        #role_name {
                            sender: sender_dual,
                        },
                        #dual_name {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#role_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }
            ////////////////////////////////////////////
            /// The normal Dual implementation of Role
            impl<R: mpstthree::role::Role> mpstthree::role::Role for #dual_name<R> {
                type Dual = #role_name<<R as mpstthree::role::Role>::Dual>;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal, _) = crossbeam_channel::bounded::<R>(1);
                    let (sender_dual, _) = crossbeam_channel::bounded::<R::Dual>(1);
                    (
                        #dual_name {
                            sender: sender_dual,
                        },
                        #role_name {
                            sender: sender_normal,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#dual_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}>",
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Role

            impl<R: mpstthree::role::Role> #role_name<R> {
                pub fn continuation(&self) -> R {
                    let (here, there) = R::new();
                    self.sender.send(there).unwrap_or(());
                    here
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Dual

            impl<R: mpstthree::role::Role> #dual_name<R> {
                pub fn continuation(&self) -> R {
                    let (here, there) = R::new();
                    self.sender.send(there).unwrap_or(());
                    here
                }
            }

            ////////////////////////////////////////////
            /// The all Role

            #[derive(Debug)]
            struct #role_to_all_name<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The all Dual

            #[derive(Debug)]
            struct #dual_to_all_name<R1, R2>
            where
                R1: mpstthree::role::Role,
                R2: mpstthree::role::Role,
                R1::Dual: mpstthree::role::Role,
                R2::Dual: mpstthree::role::Role,
            {
                sender1: crossbeam_channel::Sender<R1::Dual>,
                sender2: crossbeam_channel::Sender<R2::Dual>,
            }

            ////////////////////////////////////////////
            /// The all Role implementation of Role
            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
                for #role_to_all_name<R1, R2>
            {
                type Dual = #dual_to_all_name<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);
                    (
                        #role_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #dual_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#role_to_all_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#role_to_all_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The all Dual implementation of Role
            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> mpstthree::role::Role
                for #dual_to_all_name<R1, R2>
            {
                type Dual = #role_to_all_name<
                    <R1 as mpstthree::role::Role>::Dual,
                    <R2 as mpstthree::role::Role>::Dual,
                >;
                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    let (sender_normal_1, _) = crossbeam_channel::bounded::<R1>(1);
                    let (sender_normal_2, _) = crossbeam_channel::bounded::<R2>(1);
                    let (sender_dual_1, _) = crossbeam_channel::bounded::<R1::Dual>(1);
                    let (sender_dual_2, _) = crossbeam_channel::bounded::<R2::Dual>(1);
                    (
                        #dual_to_all_name {
                            sender1: sender_dual_1,
                            sender2: sender_dual_2,
                        },
                        #role_to_all_name {
                            sender1: sender_normal_1,
                            sender2: sender_normal_2,
                        },
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    String::from(stringify!(#dual_to_all_name))
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    String::from(stringify!(#dual_to_all_name))
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    format!(
                        "{}<{}> + {}<{}>",
                        <R1 as mpstthree::role::Role>::head_str(),
                        <R1 as mpstthree::role::Role>::tail_str(),
                        <R2 as mpstthree::role::Role>::head_str(),
                        <R2 as mpstthree::role::Role>::tail_str()
                    )
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Role

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> #role_to_all_name<R1, R2> {
                pub fn continuation_left(&self) -> R1 {
                    let (here, there) = R1::new();
                    self.sender1.send(there).unwrap_or(());
                    here
                }
                pub fn continuation_right(&self) -> R2 {
                    let (here, there) = R2::new();
                    self.sender2.send(there).unwrap_or(());
                    here
                }
            }

            ////////////////////////////////////////////
            /// The associated functions for Dual

            impl<R1: mpstthree::role::Role, R2: mpstthree::role::Role> #dual_to_all_name<R1, R2> {
                pub fn continuation_left(&self) -> R1 {
                    let (here, there) = R1::new();
                    self.sender1.send(there).unwrap_or(());
                    here
                }
                pub fn continuation_right(&self) -> R2 {
                    let (here, there) = R2::new();
                    self.sender2.send(there).unwrap_or(());
                    here
                }
            }
        }
    }

    fn expand_fork_mpst(&self) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();
        let (_diag, matrix) = self.diag_and_matrix();
        let (diag_w_offset, matrix_w_offset) = self.diag_and_matrix_w_offset();

        let sessions: Vec<TokenStream> = (1..=((self.number_roles - 1) * (self.number_roles) / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let sessions_struct: Vec<TokenStream> =
            (1..=((self.number_roles - 1) * (self.number_roles) / 2))
                .map(|i| {
                    let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                    quote! {
                        #temp_ident : mpstthree::binary::struct_trait::session::Session + 'static ,
                    }
                })
                .collect();

        let roles: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_roles: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("R{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("role_{}", i), Span::call_site());
                quote! {
                    let ( #temp_role , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let names: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let names_struct: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_ident : mpstthree::role::Role + 'static ,
                }
            })
            .collect();

        let new_names: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("N{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                quote! {
                    let ( #temp_name , _) = #temp_ident::new() ;
                }
            })
            .collect();

        let functions: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{}", i), Span::call_site());
                quote! {
                    #temp_ident ,
                }
            })
            .collect();

        let functions_detail: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("F{}", i), Span::call_site());
                let temp_expr = Ident::new(&format!("f{}", i), Span::call_site());
                quote! {
                    #temp_expr : #temp_ident ,
                }
            })
            .collect();

        let functions_struct: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles)
                    .map(|j| {
                        let (k, _, m) = self.get_tuple_matrix(&matrix_w_offset, i, j);
                        let temp_ident =
                            Ident::new(&format!("S{}", m), Span::call_site());
                        if k == i {
                            quote! {
                                #temp_ident ,
                            }
                        } else {
                            quote! {
                                < #temp_ident  as mpstthree::binary::struct_trait::session::Session>::Dual ,
                            }
                        }
                    })
                    .collect();

                let temp_function =
                    Ident::new(&format!("F{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("R{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("N{}", i), Span::call_site());
                quote! {
                    #temp_function : FnOnce(
                        #meshedchannels_name<
                            #(
                                #temp_sessions
                            )*
                            #temp_role ,
                            #temp_name
                        >
                    ) -> Result<(), Box<dyn std::error::Error>>
                    + std::marker::Send
                    + 'static,
                }
            })
            .collect();

        let join_handle: Vec<TokenStream> = (1..=self.number_roles)
            .map(|_| {
                quote! {
                    std::thread::JoinHandle<()> ,
                }
            })
            .collect();

        let new_channels: Vec<TokenStream> = (1..=((self.number_roles - 1) * (self.number_roles)
            / 2))
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                let (line, column, _) = self.get_tuple_diag(&diag_w_offset, i);
                let temp_channel_left =
                    Ident::new(&format!("channel_{}_{}", line, column), Span::call_site());
                let temp_channel_right =
                    Ident::new(&format!("channel_{}_{}", column, line), Span::call_site());
                quote! {
                    let ( #temp_channel_left , #temp_channel_right ) =
                        < #temp_ident as mpstthree::binary::struct_trait::session::Session>::new();
                }
            })
            .collect();

        let new_meshedchannels: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_sessions: Vec<TokenStream> = (1..self.number_roles)
                    .map(|j| {
                        let (line, column, _) = self.get_tuple_matrix(&matrix, i, j);
                        let temp_session = Ident::new(&format!("session{}", j), Span::call_site());
                        let temp_channel = match line {
                            m if m == i => Ident::new(
                                &format!("channel_{}_{}", line, column),
                                Span::call_site(),
                            ),
                            _ => Ident::new(
                                &format!("channel_{}_{}", column, line),
                                Span::call_site(),
                            ),
                        };
                        quote! {
                            #temp_session : #temp_channel ,
                        }
                    })
                    .collect();

                let temp_meshedchannels =
                    Ident::new(&format!("meshedchannels_{}", i), Span::call_site());
                let temp_role = Ident::new(&format!("role_{}", i), Span::call_site());
                let temp_name = Ident::new(&format!("name_{}", i), Span::call_site());
                quote! {
                    let #temp_meshedchannels =
                        #meshedchannels_name {
                            #(
                                #temp_sessions
                            )*
                            stack: #temp_role ,
                            name: #temp_name ,
                        };
                }
            })
            .collect();

        let new_threads: Vec<TokenStream> = (1..=self.number_roles)
            .map(|i| {
                let temp_function =
                    Ident::new(&format!("f{}", i), Span::call_site());
                let temp_meshedchannels = Ident::new(
                    &format!("meshedchannels_{}", i),
                    Span::call_site(),
                );
                quote! {
                    std::thread::Builder::new().name(String::from(stringify!(#temp_function))).stack_size(32 * 1024 * 1024).spawn(move || {
                        std::panic::set_hook(Box::new(|_info| {
                            // do nothing
                        }));
                        match #temp_function(#temp_meshedchannels) {
                            Ok(()) => (),
                            Err(e) => panic!("{:?}", e),
                        }
                    }).unwrap(),
                }
            })
            .collect();

        quote! {
            fn fork_mpst<
                #(
                    #sessions
                )*
                #(
                    #roles
                )*
                #(
                    #names
                )*
                #(
                    #functions
                )*
            >(
                #(
                    #functions_detail
                )*
            ) -> (
                #(
                    #join_handle
                )*
            )
            where
                #(
                    #roles_struct
                )*
                #(
                    #names_struct
                )*
                #(
                    #sessions_struct
                )*
                #(
                    #functions_struct
                )*
            {
                #(
                    #new_channels
                )*

                #(
                    #new_roles
                )*

                #(
                    #new_names
                )*

                #(
                    #new_meshedchannels
                )*

                (
                    #(
                        #new_threads
                    )*
                )
            }
        }
    }

    fn expand(&self) -> TokenStream {
        let meshedchannels_name = self.meshedchannels_name.clone();

        // Get all the roles provided into a Vec
        let all_roles = self.all_roles.clone();

        let quote_fork_mpst = self.expand_fork_mpst();

        let session_types: Vec<Ident> = (1..self.number_roles)
            .map(|i| Ident::new(&format!("S{}", i), Span::call_site()))
            .collect();

        let session_types_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { #temp_ident : mpstthree::binary::struct_trait::session::Session , }
            })
            .collect();

        let session_types_dual_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! { <#temp_ident as mpstthree::binary::struct_trait::session::Session>::Dual , }
            })
            .collect();

        let session_types_pub: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { pub #temp_session : #temp_type , }
            })
            .collect();

        let sender_receiver: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
                let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
                let temp_type = Ident::new(&format!("S{}", i), Span::call_site());
                quote! { let ( #temp_sender , #temp_receiver ) =
                <#temp_type as mpstthree::binary::struct_trait::session::Session>::new() ; }
            })
            .collect();

        let sender_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_sender = Ident::new(&format!("sender{}", i), Span::call_site());
                quote! { #temp_session : #temp_sender , }
            })
            .collect();

        let receiver_struct: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                let temp_receiver = Ident::new(&format!("receiver{}", i), Span::call_site());
                quote! { #temp_session : #temp_receiver , }
            })
            .collect();

        let head_str: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    if result.is_empty() {
                        result = format!(
                            "{}",
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str()
                        ) ;
                    } else {
                        result = format!(
                            "{}\n{}",
                            result,
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str()
                        );
                    }
                }
            })
            .collect();

        let tail_str: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_ident =
                    Ident::new(&format!("S{}", i), Span::call_site());
                quote! {
                    if result.is_empty() {
                        result = format!(
                            "{}<{}>",
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str(),
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::tail_str()
                        ) ;
                    } else {
                        result = format!(
                            "{}\n{}<{}>",
                            result,
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::head_str(),
                            <#temp_ident as mpstthree::binary::struct_trait::session::Session>::tail_str()
                        ) ;
                    }
                }
            })
            .collect();

        let stringify: Vec<TokenStream> = (1..self.number_roles)
            .map(|i| {
                let temp_session = Ident::new(&format!("session{}", i), Span::call_site());
                quote! { stringify!( #temp_session ) , }
            })
            .collect();

        let roles_struct: Vec<TokenStream> = all_roles
            .iter()
            .map(|i| self.expand_role(format!("{}", i)))
            .collect();

        let send_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| {
                (1..=self.number_roles)
                    .filter_map(|receiver| {
                        if sender != receiver {
                            Some(self.expand_send(
                                all_roles.clone(),
                                sender,
                                receiver,
                                session_types.clone(),
                                session_types_struct.clone(),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(self.expand_recv(
                                all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let recv_from_all_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(self.expand_recv_from_all(
                                all_roles.clone(),
                                receiver,
                                sender,
                                session_types.clone(),
                                session_types_struct.clone(),
                            ))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let offer_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|receiver| {
                (1..=self.number_roles)
                    .filter_map(|sender| {
                        if receiver != sender {
                            Some(self.expand_offer(all_roles.clone(), sender, receiver))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();

        let choose_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| self.expand_choose(all_roles.clone(), sender))
            .collect();

        let close_methods: Vec<TokenStream> = (1..=self.number_roles)
            .map(|sender| self.expand_close(all_roles.clone(), sender))
            .collect();

        let cancel_method: TokenStream = self.expand_cancel();

        quote! {
            #[must_use]
            #[derive(Debug)]
            pub struct #meshedchannels_name<
                #( #session_types , )*
                R,
                N
            >
            where
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            {
                #( #session_types_pub )*
                pub stack: R,
                pub name: N,
            }
            #[doc(hidden)]
            impl<
                #( #session_types_struct )*
                R: mpstthree::role::Role,
                N: mpstthree::role::Role
            > mpstthree::binary::struct_trait::session::Session for #meshedchannels_name<
                #(
                    #session_types , )*
                    R,
                    N
                > {
                type Dual =
                #meshedchannels_name<
                    #( #session_types_dual_struct )*
                    <R as mpstthree::role::Role>::Dual,
                    <N as mpstthree::role::Role>::Dual,
                >;

                #[doc(hidden)]
                fn new() -> (Self, Self::Dual) {
                    #( #sender_receiver )*

                    let (role_one, role_two) = R::new();
                    let (name_one, name_two) = N::new();
                    (
                        #meshedchannels_name {
                            #( #sender_struct )*
                            stack: role_one,
                            name: name_one,
                        },
                        #meshedchannels_name {
                            #( #receiver_struct )*
                            stack: role_two,
                            name: name_two,
                        }
                    )
                }

                #[doc(hidden)]
                fn head_str() -> String {
                    let mut result = "".to_string();
                    #( #head_str )*
                    format!(
                        "{}\n{}\n{}",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::head_str()
                    )
                }

                #[doc(hidden)]
                fn tail_str() -> String {
                    let mut result = "".to_string();
                    #( #tail_str )*
                    format!(
                        "{}\n{}<{}>\n{}<{}>",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str(),
                        <N as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::tail_str()
                    )
                }

                #[doc(hidden)]
                fn self_head_str(&self) -> String {
                    let mut result = "".to_string();
                    #( #head_str )*
                    format!(
                        "{}\n{}\n{}",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::head_str()
                    )
                }

                #[doc(hidden)]
                fn self_tail_str(&self) -> String {
                    let mut result = "".to_string();
                    #( #tail_str )*
                    format!(
                        "{}\n{}<{}>\n{}<{}>",
                        result,
                        <R as mpstthree::role::Role>::head_str(),
                        <R as mpstthree::role::Role>::tail_str(),
                        <N as mpstthree::role::Role>::head_str(),
                        <N as mpstthree::role::Role>::tail_str()
                    )
                }
            }
            #[doc(hidden)]
            impl<
                    #( #session_types_struct )*
                    R: mpstthree::role::Role,
                    N: mpstthree::role::Role
                > #meshedchannels_name<#( #session_types , )* R, N> {
                #[doc(hidden)]
                pub fn field_names(self) ->
                    (
                        &'static [&'static str],
                        #meshedchannels_name<#( #session_types , )* R, N>
                    ) {
                    (
                        &[
                            #( #stringify )*
                        ],
                        self
                    )
                }
            }


            #( #roles_struct )*

            #( #send_methods )*

            #( #recv_methods )*

            #( #recv_from_all_methods )*

            #( #offer_methods )*

            #( #choose_methods )*

            #( #close_methods )*

            #cancel_method

            #[allow(unused_macros)]
            macro_rules! offer_cancel_mpst {
                ($session: expr, { $( $pat: pat => $result: expr, )+ }) => {
                    (move || -> Result<_, _> {
                        let ((session1, cont), s) = $session.recv()?;
                        let s = s.session1.sender.send(mpstthree::binary::struct_trait::end::Signal::Offer(session1)).unwrap();
                        mpstthree::binary::cancel::cancel(s);
                        match cont {
                            $(
                                $pat => $result,
                            )+
                            _ => panic!("Unexpected payload") ,
                        }
                    })()
                };
            }

            #quote_fork_mpst

        }
    }
}
