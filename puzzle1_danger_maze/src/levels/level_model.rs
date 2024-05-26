use serde::*;
use std::vec::Vec;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelModel {
    pub player: LevelPlayerModel,
    pub goal: LevelGoalModel,
    pub danger_blocks: Vec::<LevelBlockModel>
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
