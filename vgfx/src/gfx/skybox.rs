use super::texture::Texture;

pub struct Skybox {
    pub left: Texture,
    pub right: Texture,
    pub top: Texture,
    pub bottom: Texture,
    pub front: Texture,
    pub back: Texture
}

impl Skybox {
    pub fn new() -> Self {
        return Skybox {
            left: Texture::new("left".to_string()),
            right: Texture::new("right".to_string()),
            top: Texture::new("top".to_string()),
            bottom: Texture::new("bottom".to_string()),
            front: Texture::new("front".to_string()),
            back: Texture::new("back".to_string())
        }
    }
}