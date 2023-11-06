use syn::parse::Parse;

struct FourCC {
    literal: syn::LitStr,
}

impl Parse for FourCC {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let literal = input.parse()?;
        Ok(Self { literal })
    }
}

impl FourCC {
    fn to_i32(&self) -> syn::Result<i32> {
        let value = self.literal.value();
        let bytes = value.as_bytes();
        let mut result = 0i32;

        if bytes.len() != 4 {
            return Err(syn::Error::new(self.literal.span(), "expected 4 bytes"));
        }

        for (idx, c) in bytes.iter().rev().enumerate() {
            result |= (*c as i32) << (idx * 8);
        }

        Ok(result)
    }
}

pub fn cc(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let input = syn::parse2::<FourCC>(input).unwrap();

    let value = input.to_i32().unwrap();
    let literal = syn::LitInt::new(&format!("{}i32", value), input.literal.span());

    let result = quote::quote! {
        #literal
    };

    result
}
