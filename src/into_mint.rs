#[allow(missing_docs)]
pub trait IntoMint: Into<Self::MintType> {
    type MintType;

    #[inline]
    fn into_mint(self) -> Self::MintType {
        self.into()
    }
}
