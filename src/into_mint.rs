#[allow(missing_docs)]
pub trait IntoMint: Into<Self::MintType> {
    type MintType;
}
