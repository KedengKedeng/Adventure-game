pub struct Entity {
    pub name: String,
    pub entity_id: i32,

    pub current_health: i32,
    pub max_health: i32,

    pub current_mana: i32,
    pub max_mana: i32,

    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

impl Entity {
    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn defend(&self, damage: i32) -> i32 {
        damage - self.defense
    }
}
