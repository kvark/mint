/// Defines which mint type a given type is associated with. This trait enables
/// converting a type into its mint equivalent without having to name the mint
/// type.
///
/// Implementing `IntoMint` on a type states that the type is semantically
/// equivalent to the type given in `MintType`.
///
/// All mint types implement `IntoMint` reflexively, i.e., they implement
/// `IntoMint<MintType = Self>`.
pub trait IntoMint: Into<Self::MintType> {
    /// The mint type that this type is associated with.
    type MintType;
}
