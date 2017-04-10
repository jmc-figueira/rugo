use tcod::colors::Color;
use tcod::colors::lerp;

pub type Rgb = (u8, u8, u8);

pub type Light = (Color, f32);
pub type LightSource = (i32, i32, Light);

pub trait LightingGroup{
    fn find_by_coords(&self, x: i32, y: i32) -> Vec<LightSource>;
}

impl LightingGroup for Vec<LightSource>{
    fn find_by_coords(&self, x: i32, y: i32) -> Vec<LightSource>{
        let mut ret_val = Vec::new();

        for source in self{
            if source.0 == x && source.1 == y{
                ret_val.push(*source);
            }
        }

        ret_val
    }
}

pub const DARK: Rgb = (0, 0, 0);
pub const CAVE_WALL: Rgb = (122, 82, 48);
pub const CAVE_FLOOR: Rgb = (85, 57, 33);
pub const STAIRS: Rgb = (114, 123, 132);
pub const MEMORY: Rgb = (64, 64, 64);
pub const PLAYER: Rgb = (67, 179, 174);
pub const HUD: Rgb = (96, 117, 98);
pub const OLD_MESSAGES: Rgb = (63, 77, 64);

pub struct ColorCell{
    background: Color,
    foreground: Color,
}

impl ColorCell{
    pub fn new(background: Rgb, foreground: Rgb) -> ColorCell{
        let (br, bg, bb) = background;
        let (fr, fg, fb) = foreground;
        ColorCell{
            background: Color::new(br, bg, bb),
            foreground: Color::new(fr, fg, fb)
        }
    }

    pub fn background(&self) -> &Color{
        &self.background
    }

    pub fn foreground(&self) -> &Color{
        &self.foreground
    }

    pub fn blend_light(&self, light: &Color, intensity: f32) -> ColorCell{
        let tile_color = (lerp(self.background, *light, 0.25), lerp(self.foreground, *light, 0.25));
        ColorCell{
            background: lerp(Color::new(0, 0, 0), tile_color.0, intensity.min(1f32).max(0f32)),
            foreground: lerp(Color::new(0, 0, 0), tile_color.1, intensity.min(1f32).max(0f32))
        }
    }
}

impl Clone for ColorCell{
    fn clone(&self) -> ColorCell{
        ColorCell{
            background: self.background,
            foreground: self.foreground
        }
    }
}
