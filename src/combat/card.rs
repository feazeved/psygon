#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardId {
    Strike,
    Shield,
}

#[derive(Clone, Copy)]
pub enum Effect {
    Damage(i32),
    Defense(i32),
    Heal(i32),
}

#[derive(Clone, Copy)]
pub struct Card {
    pub id: CardId,
    pub name: &'static str,
    pub cost: i32,
    pub effect: Effect,
}
