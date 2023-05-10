use proc_macro2::TokenStream;
use quote::quote;

use crate::game::tokens::get_tr_with_fallback;

use super::{GameFieldReceiver, GameLabel, InfoFieldData};

pub(crate) fn map_game_field_to_extras_value(
	(field, label): &(&GameFieldReceiver, &GameLabel),
) -> TokenStream {
	let name = field.ident.as_ref();
	let tr = get_tr_with_fallback(label.tr.as_deref(), name);

	let colour = &label.colour;
	let percent = if label.percent == Some(true) {
		quote! { true }
	} else {
		quote! { false }
	};

	let value = if let Some(div) = label.div.as_ref() {
		quote! { ::std::cmp::min(100, stats.#name * 100 / if stats.#div == 0 { 1 } else { stats.#div }) }
	} else {
		quote! { stats.#name }
	};

	quote! {
		(
			::translate::tr!(ctx, #tr),
			::std::boxed::Box::new(#value),
			#colour,
			#percent,
		),
	}
}

pub(crate) fn map_info_field_to_extras_value(info: &InfoFieldData) -> TokenStream {
	let name = &info.ident;
	let tr = get_tr_with_fallback(info.tr.as_deref(), Some(name));

	let colour = &info.colour;
	let percent = if info.percent == Some(true) {
		quote! { true }
	} else {
		quote! { false }
	};

	let value = if let Some(div) = info.div.as_ref() {
		quote! { ::std::cmp::min(100, self.#name * 100 / if self.#div == 0 { 1 } else { self.#div }) }
	} else {
		quote! { self.#name }
	};

	quote! {
		(
			::translate::tr!(ctx, #tr),
			::std::boxed::Box::new(#value),
			#colour,
			#percent,
		),
	}
}
