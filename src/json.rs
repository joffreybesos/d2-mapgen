
use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct SeedData
{
    pub seed: u32,
    pub difficulty: u32,
    pub levels: Vec<LevelData>,
}

#[derive(Serialize, Default, Deserialize, Debug)]
pub struct LevelData {
    pub id: u32,
    pub name: String,
    pub offset: Offset,
    pub size: Size,
    pub objects: Vec<Object>,
    pub map: Vec<Vec<u64>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Offset {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub unit_type: String,
    pub x: u32,
    pub y: u32,
     #[serde(skip_serializing_if = "Option::is_none")]
    pub is_good_exit: Option<bool>
}

