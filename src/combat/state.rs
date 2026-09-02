use crate::combat::card::{Card, Effect};

pub struct Combat {
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub defense: i32,
    pub enemy_hp: i32,
    pub enemy_max_hp: i32,
    pub enemy_intent: i32,
    deck: Vec<Card>,
    pub current_card: Option<Card>,
}

impl Combat {
    pub fn new(
        deck: Vec<Card>,
        player_max_hp: i32,
        max_mana: i32,
        enemy_max_hp: i32,
        enemy_base_damage: i32,
    ) -> Self {
        let mut combat = Self {
            player_hp: player_max_hp,
            player_max_hp,
            mana: max_mana,
            max_mana,
            defense: 0,
            enemy_hp: enemy_max_hp,
            enemy_max_hp,
            enemy_intent: enemy_base_damage,
            deck,
            current_card: None,
        };
        combat.draw_card();
        combat
    }

    pub fn draw_card(&mut self) {
        if self.current_card.is_none() {
            self.current_card = self.deck.pop();
        }
    }

    pub fn use_current_card(&mut self) -> bool {
        let Some(card) = self.current_card else {
            return false;
        };
        if self.mana < card.cost {
            return false;
        }
        self.mana -= card.cost;
        match card.effect {
            Effect::Damage(n) => self.enemy_hp = (self.enemy_hp - n).max(0),
            Effect::Defense(n) => self.defense += n,
            Effect::Heal(n) => self.player_hp = (self.player_hp + n).min(self.player_max_hp),
        }
        self.current_card = None;
        true
    }

    pub fn end_round(&mut self) {
        let incoming = (self.enemy_intent - self.defense).max(0);
        self.player_hp -= incoming;
        self.defense = 0;
        self.mana = self.max_mana;
        self.draw_card();
    }

    pub fn is_player_dead(&self) -> bool {
        self.player_hp <= 0
    }
    pub fn is_enemy_dead(&self) -> bool {
        self.enemy_hp <= 0
    }
}
