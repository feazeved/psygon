use crate::combat::{Card, CardId, Effect};

pub const STRIKE: Card = Card {
    id: CardId::Strike,
    name: "Strike",
    cost: 1,
    effect: Effect::Damage(3),
};
pub const SHIELD: Card = Card {
    id: CardId::Shield,
    name: "Shield",
    cost: 2,
    effect: Effect::Defense(2),
};
