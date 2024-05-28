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

impl LevelModel {
    pub fn new() -> Self {
        LevelModel {
            player: LevelPlayerModel{x:0.0,y:0.0},
            goal: LevelGoalModel{x:0.0, y:0.0},
            danger_blocks: Vec::<LevelBlockModel>::new(),
            danger_circles: Vec::<LevelCircleModel>::new(),
            images: Vec::<LevelImageModel>::new()
        }
    }
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

impl LevelBlockModel {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        LevelBlockModel {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelCircleModel {
    pub x: f64,
    pub y: f64,
    pub r: f64
}

impl LevelCircleModel {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        LevelCircleModel {
            x: x,
            y: y,
            r: r
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LevelImageModel {
    pub filename: String,
    pub x: f64,
    pub y: f64
}
