use proc_macro2::TokenStream as TokenStream2;
use syn::{spanned::Spanned, Data, DeriveInput, Generics, Ident};

mod attr;
mod bounds;
mod from;
mod repr;
mod to;

use bounds::extend_bounds;
use repr::{Repr, VariantRepr};

pub(crate) struct DeriveData {
    pub(crate) ident: Ident,
    pub(crate) repr: Repr,
    pub(crate) generics: Generics,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum Direction {
    To,
    From,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ToVariantTrait {
    ToVariant,
    OwnedToVariant,
}

impl ToVariantTrait {
    fn trait_path(self) -> syn::Path {
        match self {
            Self::ToVariant => parse_quote! { ::gdnative::core_types::ToVariant },
            Self::OwnedToVariant => parse_quote! { ::gdnative::core_types::OwnedToVariant },
        }
    }

    fn to_variant_fn(self) -> syn::Ident {
        match self {
            Self::ToVariant => parse_quote! { to_variant },
            Self::OwnedToVariant => parse_quote! { owned_to_variant },
        }
    }

    fn to_variant_receiver(self) -> syn::Receiver {
        match self {
            Self::ToVariant => parse_quote! { &self },
            Self::OwnedToVariant => parse_quote! { self },
        }
    }
}

pub(crate) fn parse_derive_input(
    input: DeriveInput,
    bound: &syn::Path,
    dir: Direction,
) -> Result<DeriveData, syn::Error> {
    let repr = match input.data {
        Data::Struct(struct_data) => Repr::Struct(VariantRepr::repr_for(&struct_data.fields)?),
        Data::Enum(enum_data) => Repr::Enum(
            enum_data
                .variants
                .iter()
                .map(|variant| {
                    Ok((
                        variant.ident.clone(),
                        VariantRepr::repr_for(&variant.fields)?,
                    ))
                })
                .collect::<Result<_, syn::Error>>()?,
        ),
        Data::Union(_) => {
            return Err(syn::Error::new(
                input.span(),
                "Variant conversion derive macro does not work on unions.",
            ))
        }
    };

    let generics = extend_bounds(input.generics, &repr, bound, dir);

    Ok(DeriveData {
        ident: input.ident,
        repr,
        generics,
    })
}

pub(crate) fn derive_to_variant(
    trait_kind: ToVariantTrait,
    input: proc_macro::TokenStream,
) -> Result<TokenStream2, syn::Error> {
    let derive_input = syn::parse_macro_input::parse::<syn::DeriveInput>(input)?;

    let variant = to::expand_to_variant(
        trait_kind,
        parse_derive_input(derive_input, &trait_kind.trait_path(), Direction::To)?,
    )?;

    Ok(variant)
}

pub(crate) fn derive_from_variant(derive_input: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let bound: syn::Path = syn::parse_quote! { ::gdnative::core_types::FromVariant };

    let variant = parse_derive_input(derive_input, &bound, Direction::From);
    from::expand_from_variant(variant?)
}
