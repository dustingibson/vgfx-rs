use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ModelInstance {
    pub model_name: String,
    pub position: Vec<f32>
}

#[derive(Serialize, Deserialize)]
pub struct AreaInstance {
    pub color_polygons: Vec<ColorPolygon>,
    pub texture_polygons: Vec<TexturePolygon>,
    pub model_instances: Vec<ModelInstance>
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub submodels: Vec<SubModel>
}

#[derive(Serialize, Deserialize)]
pub struct SubModel {
    pub name: String,
    pub color_polygons: Vec<ColorPolygon>,
    pub texture_polygons: Vec<TexturePolygon>,
}

#[derive(Serialize, Deserialize)]
pub struct ColorPolygon {
    pub vertices: Vec<f32>,
    pub color: Vec<f32>
}

#[derive(Serialize, Deserialize)]
pub struct TexturePolygon {
    pub vertices: Vec<f32>,
    pub texture_name: String
}