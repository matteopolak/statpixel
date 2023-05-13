#![feature(let_chains)]

mod diff;
mod game;
mod mode;

pub(crate) mod sum;
pub(crate) mod tokens;

use diff::DiffInputReceiver;
use game::GameInputReceiver;
use mode::ModeInputReceiver;

use darling::FromDeriveInput;
use quote::ToTokens;
use syn::parse_macro_input;

#[proc_macro_derive(Game, attributes(game))]
pub fn derive_game(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = GameInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}

#[proc_macro_derive(Mode, attributes(mode))]
pub fn derive_mode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = ModeInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}

#[proc_macro_derive(Diff)]
pub fn derive_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as syn::DeriveInput);
	let receiver = DiffInputReceiver::from_derive_input(&input);

	match receiver {
		Ok(receiver) => receiver.into_token_stream().into(),
		Err(error) => error.write_errors().into(),
	}
}
