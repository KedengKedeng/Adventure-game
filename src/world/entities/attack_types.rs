use super::{afflictions::Affliction, elemental_types::ElementalTypes};


pub struct Attack {
    pub damage: i32,
    pub elemental_type: ElementalTypes,

    pub attacker: i32,
    pub target: i32,

    pub affliction: Option<Affliction>
}

pub enum AttackTypes {
    Base
}
