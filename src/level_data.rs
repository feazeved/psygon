pub struct LevelData {
    pub name: &'static str,
    pub background_index: usize,
    pub enemy_max_health: i32,
    pub enemy_base_damage: i32,
}

pub const LEVELS: [LevelData; 10] = [
    LevelData {
        name: "Level 1",
        background_index: 0,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 2",
        background_index: 1,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 3",
        background_index: 2,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 4",
        background_index: 3,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 5",
        background_index: 4,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 6",
        background_index: 5,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 7",
        background_index: 6,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 8",
        background_index: 7,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 9",
        background_index: 8,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
    LevelData {
        name: "Level 10",
        background_index: 9,
        enemy_max_health: 4,
        enemy_base_damage: 1,
    },
];
