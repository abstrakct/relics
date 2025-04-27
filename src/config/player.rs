use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlayerConfig {
    pub name: String,
    pub hp: u32,
    pub mp: u32,
    pub strength: i32,
    pub con: i32,
    pub int: i32,
    pub dex: i32,
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
            strength: 0,
            con: 0,
            int: 0,
            dex: 0,
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
