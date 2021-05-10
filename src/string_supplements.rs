#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringConstrained<
    const MAX_UTF8_BYTES: usize,
    const ALLOW_BLANK: bool,
    const ALLOW_EMPTY: bool,
> {
    inner: String,
}

impl<const MAX_UTF8_BYTES: usize, const ALLOW_BLANK: bool, const ALLOW_EMPTY: bool>
    StringConstrained<MAX_UTF8_BYTES, ALLOW_BLANK, ALLOW_EMPTY>
{
    pub fn new(inner: String) -> Self {
        assert!(MAX_UTF8_BYTES >= inner.len());
        assert!(ALLOW_EMPTY || !inner.is_empty());
        assert!(
            ALLOW_BLANK
                || match inner.find(|c: char| !c.is_whitespace()) {
                    None => false,
                    _ => true,
                }
        );
        StringConstrained { inner }
    }
}
