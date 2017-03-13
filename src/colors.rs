use tcod::colors::Color;
use tcod::colors::lerp;

pub struct ColorCell{
    background: Color,
    foreground: Color,
}

impl ColorCell{
    pub fn new(background: (u8, u8, u8), foreground: (u8, u8, u8)) -> ColorCell{
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

    pub fn blend_light(&self, light: &Color, intensity: f32, absorption: f32, dx: i32, dy: i32) -> ColorCell{
        let absorbed = ((intensity - absorption * ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt()).min(1f32)).max(0f32);
        let tile_color = (lerp(self.background, *light, 0.5), lerp(self.foreground, *light, 0.5));
        ColorCell{
            background: lerp(Color::new(0, 0, 0), tile_color.0, absorbed),
            foreground: lerp(Color::new(0, 0, 0), tile_color.1, absorbed)
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
