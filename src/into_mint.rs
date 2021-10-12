#[allow(missing_docs)]
pub trait IntoMint: Into<Self::MintType> {
    type MintType;

    #[inline]
    fn into_mint(self) -> Self::MintType {
        self.into()
    }
}

macro_rules! into_mint_identity {
    ($($ty:ident),*) => {
        $(
            impl IntoMint for $ty {
                type MintType = $ty;
            }
        )*
    };
}

into_mint_identity!(f32, f64);
into_mint_identity!(u8, u16, u32, u64, u128);
into_mint_identity!(i8, i16, i32, i64, i128);
