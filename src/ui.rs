use tcod::console::*;
use stats::Stats;
use colors::*;

const DEFAULT_HEIGHT: i32 = 5;

pub trait SystemUI{
    fn show_hud(&self) -> &Offscreen;

    fn update_hud(&mut self, stats: Stats);
}

pub struct SciUI{
    hud: Offscreen,
    hud_color: ColorCell,
    pub hud_width: i32,
    pub hud_height: i32,
}

impl SystemUI for SciUI{
    fn show_hud(&self) -> &Offscreen{
        &self.hud
    }

    fn update_hud(&mut self, stats: Stats){
        let hp_bar_width = self.hud_width / 4;

        let initial_hp = "HP: [";
        let initial_len = initial_hp.len() as i32;
        let hp_text = format!("] ({}/{})", stats.curr_hp, stats.max_hp);
        let hp_len = hp_text.len() as i32;

        self.hud.print_ex(2, 2, BackgroundFlag::None, TextAlignment::Left, initial_hp);

        self.hud.print_ex(2 + initial_hp.len() as i32 + hp_bar_width, 2, BackgroundFlag::None, TextAlignment::Left, hp_text);

        let curr_hp_width = ((stats.curr_hp * hp_bar_width as u64) / stats.max_hp) as i32;

        self.hud.set_default_background(*self.hud_color.background());
        self.hud.rect(2 + initial_len, 2, curr_hp_width, 1, false, BackgroundFlag::Set);
        self.hud.set_default_background(*self.hud_color.foreground());
        self.hud.rect(2 + initial_len, 2, curr_hp_width, 1, false, BackgroundFlag::Set);

        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 3, 1, BackgroundFlag::None, TextAlignment::Left, "Str: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 3, 3, BackgroundFlag::None, TextAlignment::Left, "Dex: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 8, 1, BackgroundFlag::None, TextAlignment::Left, "Con: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 8, 3, BackgroundFlag::None, TextAlignment::Left, "Int: ");
    }
}

impl SciUI{
    pub fn new(hud_width: i32) -> SciUI{
        SciUI{
            hud: SciUI::generate_hud(hud_width, DEFAULT_HEIGHT),
            hud_color: ColorCell::new(DARK, HUD),
            hud_width: hud_width,
            hud_height: DEFAULT_HEIGHT
        }
    }

    fn generate_hud(width: i32, height: i32) -> Offscreen{
        SciUI::draw_box(width, height)
    }

    fn draw_box(width: i32, height: i32) -> Offscreen{
        let mut ret_val = Offscreen::new(width + 1, height + 1);

        let hud_color = ColorCell::new(DARK, HUD);

        ret_val.set_default_foreground(*hud_color.foreground());

        ret_val.print_frame::<&str>(0, 0, width, height, false, BackgroundFlag::None, None);

        ret_val
    }
}
