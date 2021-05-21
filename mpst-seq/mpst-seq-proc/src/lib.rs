#![recursion_limit = "128"]

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::usize;
use syn::parse_macro_input;

mod functionmpst;

use functionmpst::recv_all_aux_simple::RecvAllAuxSimpleMacroInput;
use functionmpst::recv_aux_simple::RecvAuxSimpleMacroInput;
use functionmpst::send_aux_simple::SendAuxSimpleMacroInput;

mod macros;

use macros::create_broadcast_role_short::CreateBroadcastRoleShortMacroInput;
use macros::create_normal_role_short::CreateNormalRoleShortMacroInput;
use macros::multiple::broadcast_cancel::BroadcastCancelMacroInput;
use macros::multiple::send_cancel::SendCancelMacroInput;

mod basic;

use basic::basic::SeqMacroInput;

//////////////////////////////////////

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SeqMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_seq(input: TokenStream) -> TokenStream {
    seq(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn recv_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAuxSimpleMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_aux_simple(input: TokenStream) -> TokenStream {
    recv_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn recv_all_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RecvAllAuxSimpleMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_recv_all_aux_simple(input: TokenStream) -> TokenStream {
    recv_all_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn send_aux_simple(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendAuxSimpleMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_aux_simple(input: TokenStream) -> TokenStream {
    send_aux_simple(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn send_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SendCancelMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_send_cancel(input: TokenStream) -> TokenStream {
    send_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn broadcast_cancel(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as BroadcastCancelMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_broadcast_cancel(input: TokenStream) -> TokenStream {
    broadcast_cancel(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_normal_role_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateNormalRoleShortMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_normal_role_short(input: TokenStream) -> TokenStream {
    create_normal_role_short(input)
}

//////////////////////////////////////

#[proc_macro]
pub fn create_broadcast_role_short(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as CreateBroadcastRoleShortMacroInput);
    let output: proc_macro2::TokenStream = proc_macro2::TokenStream::from(input);
    output.into()
}

#[proc_macro_hack]
pub fn e_create_broadcast_role_short(input: TokenStream) -> TokenStream {
    create_broadcast_role_short(input)
}
