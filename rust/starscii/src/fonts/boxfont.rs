use crate::fonts::{FontConfig, FontFamily};
pub struct BoxFont;

impl FontFamily for BoxFont {
    fn c(&self, conf: &FontConfig, i: &u8) -> String {
        let total_rows = usize::from(conf.font_rows);
        let total_cols = usize::from(conf.font_cols);
        let current_row = usize::from(*i);
        let righ_space = " ".repeat(2);
        let top_bottom = conf.filler.to_string().repeat(total_cols - 2) + &righ_space;
        let mid_space = " ".repeat(total_cols - 1);
        let mid_str = conf.filler.to_string() + &mid_space;
        let mut c = String::from("");
        if current_row == 2 || current_row == total_rows - 1 {
            c.push_str(&top_bottom);
        } else if current_row > 2 {
            c.push_str(&mid_str);
        } else {
            let empty_space = String::from(" ") + &mid_space;
            c.push_str(&empty_space)
        }
        c
    }
    fn c_(&self, conf: &FontConfig, i: &u8) -> String {
        let total_rows = usize::from(conf.font_rows);
        let total_cols = usize::from(conf.font_cols);
        let current_row = usize::from(*i);
        let top_bottom = conf.filler.to_string().repeat(total_cols);
        let mid_space = " ".repeat(total_cols - 1);
        let mid_str = conf.filler.to_string() + &mid_space;
        let mut c = String::from("");
        if current_row == 0 || current_row == total_rows - 1 {
            c.push_str(&top_bottom);
        } else {
            c.push_str(&mid_str);
        }
        c
    }
}