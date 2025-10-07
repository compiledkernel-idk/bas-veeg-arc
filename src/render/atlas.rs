use ahash::AHashMap;
use macroquad::prelude::*;

pub struct TextureAtlas {
    pub texture: Texture2D,
    pub regions: AHashMap<String, AtlasRegion>,
}

#[derive(Clone, Debug)]
pub struct AtlasRegion {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl TextureAtlas {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            texture,
            regions: AHashMap::new(),
        }
    }

    pub fn add_region(&mut self, name: String, x: f32, y: f32, width: f32, height: f32) {
        self.regions.insert(
            name,
            AtlasRegion {
                x,
                y,
                width,
                height,
            },
        );
    }

    pub fn get_region(&self, name: &str) -> Option<&AtlasRegion> {
        self.regions.get(name)
    }

    pub fn draw_region(&self, name: &str, x: f32, y: f32, color: Color) {
        if let Some(region) = self.get_region(name) {
            draw_texture_ex(
                &self.texture,
                x,
                y,
                color,
                DrawTextureParams {
                    source: Some(Rect::new(region.x, region.y, region.width, region.height)),
                    ..Default::default()
                },
            );
        }
    }
}
