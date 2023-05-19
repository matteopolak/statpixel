use darling::{ast, FromDeriveInput, FromField, FromMeta};
use minecraft::paint::Paint;
use quote::{quote, ToTokens};

use crate::{sum, tokens::get_tr_with_fallback};

macro_rules! ident {
	($id: literal) => {
		::syn::Ident::new($id, ::proc_macro2::Span::call_site())
	};
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mode), supports(struct_named))]
pub(crate) struct ModeInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<(), ModeFieldReceiver>,
	#[darling(multiple)]
	pub field: Vec<ModeFieldData>,
}

impl ToTokens for ModeInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let ModeInputReceiver {
			ident,
			generics,
			data,
			field: field_data,
		} = self;

		let fields = data
			.as_ref()
			.take_struct()
			.expect("should be a named struct")
			.fields;

		let outer_start_idx = fields.iter().filter(|data| data.field.is_some()).count();
		let apply_items_mode = fields
			.iter()
			.filter_map(|data| data.field.as_ref().map(|field| (data, field)))
			.enumerate()
			.map(|(idx, (data, field))| {
				let ident = data.ident.as_ref().unwrap();
				let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

				let percent = match field.percent.as_ref() {
					Some(true) => quote! { Some(true) },
					_ => quote! { Some(false) },
				};
				let colour = &field.colour;
				let value = if let Some(div) = field.div.as_ref() {
					if field.percent == Some(true) {
						sum::div_u32_single_field(&ident!("self"), None, ident, div)
					} else {
						sum::div_f32_single_field(&ident!("self"), None, ident, div)
					}
				} else {
					quote! { self.#ident }
				};

				Some(quote! {
					crate::canvas::game::bubble(
						ctx,
						surface,
						#value,
						&::translate::tr!(ctx, #tr),
						#colour,
						#percent,
						#idx + start_idx,
					);
				})
			});

		let apply_field_items_mode = field_data.iter().enumerate().map(|(idx, field)| {
			let idx = idx + outer_start_idx;

			let ident = &field.ident;
			let tr = get_tr_with_fallback(field.tr.as_deref(), Some(ident));

			let percent = match field.percent.as_ref() {
				Some(true) => quote! { Some(true) },
				_ => quote! { Some(false) },
			};
			let colour = &field.colour;
			let value = if let Some(div) = field.div.as_ref() {
				if field.percent == Some(true) {
					sum::div_u32_single_field(&ident!("self"), None, ident, div)
				} else {
					sum::div_f32_single_field(&ident!("self"), None, ident, div)
				}
			} else {
				quote! { self.#ident }
			};

			quote! {
				crate::canvas::game::bubble(
					ctx,
					surface,
					#value,
					&::translate::tr!(ctx, #tr),
					#colour,
					#percent,
					#idx + start_idx,
				);
			}
		});

		let field_count = outer_start_idx as u8;

		tokens.extend(quote! {
			impl #ident #generics {
				pub fn apply_own_fields(
					&self,
					ctx: ::translate::Context<'_>,
					surface: &mut ::skia_safe::Surface,
					player: &crate::player::data::Data,
					session: &crate::player::status::Session,
					stats: &Stats,
					start_idx: usize,
				) {
					#(#apply_items_mode)*
					#(#apply_field_items_mode)*
				}

				pub fn get_own_field_count() -> u8 {
					#field_count
				}
			}
		});
	}
}

#[derive(Debug, FromField)]
#[darling(attributes(mode))]
pub(crate) struct ModeFieldReceiver {
	/// Get the ident of the field. For fields in tuple or newtype structs or
	/// enum bodies, this can be `None`.
	pub ident: Option<syn::Ident>,

	/// Field data
	pub field: Option<ModeField>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeField {
	#[darling(default)]
	colour: Paint,

	/// The translation key of the label.
	/// Defaults to the ident with underscores replaced with dashes.
	tr: Option<String>,

	div: Option<syn::Ident>,

	percent: Option<bool>,
}

#[derive(Debug, FromMeta)]
pub(crate) struct ModeFieldData {
	ident: syn::Ident,

	div: Option<syn::Ident>,

	tr: Option<String>,

	#[darling(default)]
	colour: Paint,

	percent: Option<bool>,
}
