use crate::*;

/// Filters requests based on a boolean condition.
///
/// The function continues execution only if the provided code block returns `true`.
///
/// # Arguments
///
/// - `attr`: A code block that returns a boolean value.
/// - `item`: The function to which the attribute is applied.
///
/// # Returns
///
/// - `TokenStream` - The modified function wrapped with a conditional guard;
///   the original function body is executed only if the condition is `true`,
///   otherwise the function returns early without doing anything.
pub(crate) fn filter_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let condition: Expr = parse_macro_input!(attr as Expr);
    inject(position, item, |_| {
        quote! {
            if !(#condition) {
                return;
            }
        }
    })
}

inventory::submit! {
    InjectableMacro {
        name: "filter",
        handler: Handler::WithAttrPosition(filter_macro),
    }
}
