use tcod::console::*;
use stats::Stats;
use colors::*;

pub trait SystemUI{
    fn show_hud(&self) -> &Offscreen;

    fn update_hud(&self, stats: &Stats);
}

pub struct SciUI{
    hud: Offscreen,
    pub hud_width: i32,
    pub hud_height: i32,
}

impl SystemUI for SciUI{
    fn show_hud(&self) -> &Offscreen{
        &self.hud
    }

    fn update_hud(&self, stats: &Stats){

    }
}

impl SciUI{
    pub fn new(hud_width: i32, hud_height: i32) -> SciUI{
        SciUI{
            hud: SciUI::generate_hud(hud_width, hud_height),
            hud_width: hud_width,
            hud_height: hud_height
        }
    }

    fn generate_hud(width: i32, height: i32) -> Offscreen{
        SciUI::draw_box(width, height)
    }

    fn draw_box(width: i32, height: i32) -> Offscreen{
        let mut ret_val = Offscreen::new(width + 1, height + 1);

        let hud_color = ColorCell::new(DARK, HUD);

        ret_val.set_default_background(*hud_color.background());
        ret_val.set_default_foreground(*hud_color.foreground());

        ret_val.print_frame::<&str>(0, 0, width, height, false, BackgroundFlag::None, None);

        ret_val
    }
}
