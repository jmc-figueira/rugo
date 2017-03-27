pub trait SystemUI{
    fn show_hud(&self, hud: &mut Console);

    fn update_hud(&self, &Stats, hud: &mut Console);

    fn hide_hud(&self, hud: &mut Console);
}

pub struct SciUI{
    hud_visible: bool,
}

impl SystemUI for SciUI{
    fn show_hud(&self, hud: &mut Console){
        if(!self.hud_visible){

        }
    }

    fn update_hud(&self, &Stats, hud: &mut Console){
    }

    fn hide_hud(&self, hud: &mut Console){
        if(self.hud_visible){

        }
    }
}
