pub enum AfflictionTypes {
    None,
    Burn,
    Freeze,
    Poison
}

pub struct Affliction {
    pub name: String,
    pub affliction_type: AfflictionTypes,
    pub turns_left: i32,

    pub base_damage: i32,

    pub turn_skip_chance: i8
}