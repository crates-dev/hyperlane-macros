use crate::*;

/// Implementation of Parse trait for ResponseHeaderData.
///
/// Parses header key, operation and value from input stream.
impl Parse for ResponseHeaderData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Expr = input.parse()?;
        let operation: HeaderOperation = if input.peek(Token![=>]) {
            input.parse::<Token![=>]>()?;
            HeaderOperation::Replace
        } else if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            HeaderOperation::Set
        } else {
            return Err(syn::Error::new(
                input.span(),
                "Expected either ',' for set operation or '=>' for replace operation",
            ));
        };
        let value: Expr = input.parse()?;
        Ok(ResponseHeaderData {
            key,
            value,
            operation,
        })
    }
}

/// Implementation of Parse trait for ResponseBodyData.
///
/// Parses response body expression from input stream.
impl Parse for ResponseBodyData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body: Expr = input.parse()?;
        Ok(ResponseBodyData { body })
    }
}
