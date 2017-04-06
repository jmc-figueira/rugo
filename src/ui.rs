use tcod::console::*;
use stats::Stats;
use colors::*;

const DEFAULT_HUD_HEIGHT: i32 = 5;

pub trait HUD{
    fn show_hud(&self) -> &Offscreen;

    fn update_hud(&mut self, stats: Stats);
}

pub trait SystemMessages{
    fn show_messages(&self) -> &Offscreen;

    fn print(&mut self, message: &str);
}

pub struct SciUI{
    hud: Offscreen,
    ui_color: ColorCell,
    sys_mesg: Offscreen,
    pub hud_width: i32,
    pub hud_height: i32,
    pub mesg_width: i32,
    pub mesg_height: i32,
    messages: Vec<String>,
}

impl HUD for SciUI{
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

        self.hud.set_default_background(*self.ui_color.background());
        self.hud.rect(2 + initial_len, 2, curr_hp_width, 1, false, BackgroundFlag::Set);
        self.hud.set_default_background(*self.ui_color.foreground());
        self.hud.rect(2 + initial_len, 2, curr_hp_width, 1, false, BackgroundFlag::Set);

        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 3, 1, BackgroundFlag::None, TextAlignment::Left, "Str: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 3, 3, BackgroundFlag::None, TextAlignment::Left, "Dex: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 8, 1, BackgroundFlag::None, TextAlignment::Left, "Con: ");
        self.hud.print_ex(2 + initial_len + hp_bar_width + hp_len + 8, 3, BackgroundFlag::None, TextAlignment::Left, "Int: ");
    }
}

impl SystemMessages for SciUI{
    fn show_messages(&self) -> &Offscreen {
        &self.sys_mesg
    }

    fn print(&mut self, message: &str) {
        self.messages.push(String::from(message));
        self.sys_mesg.print_rect_ex(2, 2, self.mesg_width - 4, self.mesg_height - 4, BackgroundFlag::None, TextAlignment::Left, message);
    }
}

impl SciUI{
    pub fn new(max_width: i32, max_height: i32) -> SciUI{
        SciUI{
            hud: SciUI::draw_box(max_width, DEFAULT_HUD_HEIGHT, None),
            ui_color: ColorCell::new(DARK, HUD),
            sys_mesg: SciUI::draw_box(max_width / 3, max_height / 2, Some("System Messages")),
            hud_width: max_width,
            hud_height: DEFAULT_HUD_HEIGHT,
            mesg_width: max_width / 3,
            mesg_height: max_height / 2,
            messages: Vec::new()
        }
    }

    fn draw_box(width: i32, height: i32, title: Option<&str>) -> Offscreen{
        let mut ret_val = Offscreen::new(width + 1, height + 1);

        let ui_color = ColorCell::new(DARK, HUD);

        ret_val.set_default_foreground(*ui_color.foreground());

        ret_val.print_frame::<&str>(0, 0, width, height, false, BackgroundFlag::None, title);

        ret_val
    }

    pub fn show_all(&mut self, screen_width: i32, screen_height: i32) -> Offscreen{
        let mut ret_val = SciUI::draw_box(screen_width, screen_height, Some("System Messages"));

        for message in self.messages.iter(){
            ret_val.print_rect_ex(2, 2, screen_width - 4, screen_height - 4, BackgroundFlag::None, TextAlignment::Left, message);
        }

        ret_val
    }
}
