use syn::{Result, Token};
use syn::parse::{Parse, ParseStream};
use syn::ItemImpl;
use syn::ItemFn;
use syn::DeriveInput;

#[derive(Debug)]
pub enum NodeItem {
    Function(ItemFn),
    Impl(ItemImpl),
    Derive(DeriveInput),
}

impl Parse for NodeItem {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token!(impl)) {
            input.parse().map(NodeItem::Impl)
        } else if input.fork().parse::<DeriveInput>().is_ok() {
            input.parse().map(NodeItem::Derive)
        } else {
            input.parse().map(NodeItem::Function)
        }
    }
}
