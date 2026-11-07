use horizon_mod_sdk::{
    Mod, Registration,
    game,
    hud::{self, Corner, Line},
};

struct CoordsDay {
    shown: String,
}

impl CoordsDay {
    fn update(&mut self) {
        let text = match game::get_player() {
            Some(p) => {
                let day = game::get_clock().map_or(1, |c| c.day_time / 24000 + 1);
                format!(
                    "XYZ: {} {} {} | Day: {}",
                    p.x.floor() as i64,
                    p.y.floor() as i64,
                    p.z.floor() as i64,
                    day
                )
            }
            None => String::new(),
        };
        if text == self.shown {
            return;
        }
        if text.is_empty() {
            hud::clear();
        } else {
            hud::set_panel(
                Corner::TopLeft,
                &[Line {
                    text: text.clone(),
                    color: 0xFFFFFF,
                }],
            );
        }
        self.shown = text;
    }
}

impl Mod for CoordsDay {
    fn init() -> Result<(Self, Registration), String> {
        let mut coords = CoordsDay { shown: String::new() };
        coords.update();
        Ok((coords, Registration::new().on_tick()))
    }
    fn tick(&mut self, _tick: u64) {
        self.update();
    }
}

horizon_mod_sdk::export_mod!(CoordsDay);
