use dice::*;
use colors::*;
use tcod::colors::Color;

struct Monster{
    name: String,
    graphic: char,
    color: ColorCell,
    light: Color,
    light_intensity: f32,
    hit_dice: Dice,
}