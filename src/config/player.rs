use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlayerConfig {
    pub name: String,
    pub hp: i32,
    pub mp: i32,
    pub str: String,
    pub con: String,
    pub int: String,
    pub dex: String,
    pub vision_range: i32,
    pub skills: PlayerSkills,
    pub hp_regen: PlayerRegen,
    pub mp_regen: PlayerRegen,
    pub equipment: PlayerEquipment,
}

impl PlayerConfig {
    pub fn new() -> PlayerConfig {
        PlayerConfig {
            name: "unknown".to_string(),
            hp: 0,
            mp: 0,
            str: "1d1".to_string(),
            con: "1d1".to_string(),
            int: "1d1".to_string(),
            dex: "1d1".to_string(),
            vision_range: 0,
            skills: PlayerSkills {
                melee: 0,
                defense: 0,
                magic: 0,
            },
            hp_regen: PlayerRegen { frequency: 0, amount: 0 },
            mp_regen: PlayerRegen { frequency: 0, amount: 0 },
            equipment: PlayerEquipment {
                equipped: Vec::new(),
                carried: Vec::new(),
                carried_stacks: Vec::new(),
            },
        }
    }
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize, Debug)]
pub struct PlayerSkills {
    pub melee: i32,
    pub defense: i32,
    pub magic: i32,
}

#[derive(Deserialize, Debug)]
pub struct PlayerRegen {
    pub frequency: i32,
    pub amount: i32,
}

#[derive(Deserialize, Debug)]
pub struct ItemStack {
    pub name: String,
    pub amount: i32,
}

#[derive(Deserialize, Debug)]
pub struct PlayerEquipment {
    pub equipped: Vec<String>,
    pub carried: Vec<String>,
    pub carried_stacks: Vec<ItemStack>,
}
