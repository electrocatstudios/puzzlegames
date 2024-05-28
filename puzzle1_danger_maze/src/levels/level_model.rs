use serde::*;
use std::vec::Vec;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelModel {
    pub player: LevelPlayerModel,
    pub goal: LevelGoalModel,
    pub danger_blocks: Vec::<LevelBlockModel>,
    pub danger_circles: Vec::<LevelCircleModel>,
    pub images: Vec::<LevelImageModel>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelPlayerModel {
    pub x: f64,
    pub y: f64
}


#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelGoalModel {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelBlockModel {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelCircleModel {
    pub x: f64,
    pub y: f64,
    pub r: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelImageModel {
    pub filename: String,
    pub x: f64,
    pub y: f64
}
