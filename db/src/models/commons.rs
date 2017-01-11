#[derive(Debug, Clone, Copy, ToSql, FromSql)]
#[postgres(name = "element_type")]
pub enum ElementType {
    #[postgres(name = "water")]
    Water,
    #[postgres(name = "fire")]
    Fire,
    #[postgres(name = "earth")]
    Earth,
    #[postgres(name = "electric")]
    Electric,
    #[postgres(name = "plant")]
    Plant
}
