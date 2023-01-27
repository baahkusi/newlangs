#![allow(unused_variables)]
use crate::fonts::boxfont::BoxFont;

pub mod boxfont;
pub struct FontConfig {
    pub font_size: u8,
    pub font_rows: u8,
    pub font_cols: u8,
    pub filler: char,
}

pub trait FontFamily {
    fn process_config(&self, config: &mut FontConfig)  {
    }
    fn draw(&self, conf: &mut FontConfig, w: &str) -> String {
        let mut stars = String::from("");
        self.process_config(conf);
        for row in 0..conf.font_rows {
            stars.push_str(&self.draw_row(&conf, &w, &row));
        }
        stars
    }
    fn draw_row(&self, conf: &FontConfig, w: &str, i: &u8) -> String {
        let mut row = String::from("");
        for c in w.chars() {
            row.push_str(&self.draw_char_row(&c, &conf, &i));
        }
        row.push_str("\n");
        row
    }
    fn draw_char_row(&self, c: &char, conf: &FontConfig, i: &u8) -> String {
        match c {
            'a' => self.a(conf, i),
            'b' => self.b(conf, i),
            'c' => self.c(conf, i),
            'd' => self.d(conf, i),
            'e' => self.e(conf, i),
            'f' => self.f(conf, i),
            'g' => self.g(conf, i),
            'h' => self.h(conf, i),
            'i' => self.i(conf, i),
            'j' => self.j(conf, i),
            'k' => self.k(conf, i),
            'l' => self.l(conf, i),
            'm' => self.m(conf, i),
            'n' => self.n(conf, i),
            'o' => self.o(conf, i),
            'p' => self.p(conf, i),
            'q' => self.q(conf, i),
            'r' => self.r(conf, i),
            's' => self.s(conf, i),
            't' => self.t(conf, i),
            'u' => self.u(conf, i),
            'v' => self.v(conf, i),
            'w' => self.w(conf, i),
            'x' => self.x(conf, i),
            'y' => self.y(conf, i),
            'z' => self.z(conf, i),
            'A' => self.a_(conf, i),
            'B' => self.b_(conf, i),
            'C' => self.c_(conf, i),
            'D' => self.d_(conf, i),
            'E' => self.e_(conf, i),
            'F' => self.f_(conf, i),
            'G' => self.g_(conf, i),
            'H' => self.h_(conf, i),
            'I' => self.i_(conf, i),
            'J' => self.j_(conf, i),
            'K' => self.k_(conf, i),
            'L' => self.l_(conf, i),
            'M' => self.m_(conf, i),
            'N' => self.n_(conf, i),
            'O' => self.o_(conf, i),
            'P' => self.p_(conf, i),
            'Q' => self.q_(conf, i),
            'R' => self.r_(conf, i),
            'S' => self.s_(conf, i),
            'T' => self.t_(conf, i),
            'U' => self.u_(conf, i),
            'V' => self.v_(conf, i),
            'W' => self.w_(conf, i),
            'X' => self.x_(conf, i),
            'Y' => self.y_(conf, i),
            'Z' => self.z_(conf, i),
            ' ' => String::from(" "),
            _ => String::from(""),
        }
    }
    fn a(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn b(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn c(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn d(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn e(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn f(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn g(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn h(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn i(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn j(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn k(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn l(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn m(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn n(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn o(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn p(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn q(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn r(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn s(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn t(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn u(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn v(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn w(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn x(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn y(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn z(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn a_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn b_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn c_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn d_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn e_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn f_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn g_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn h_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn i_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn j_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn k_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn l_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn m_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn n_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn o_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn p_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn q_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn r_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn s_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn t_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn u_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn v_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn w_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn x_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn y_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
    fn z_(&self, conf: &FontConfig, i: &u8) -> String {
        String::from(conf.filler)
    }
}

pub struct StarsciiConfig {
    pub font_size: u8,
    pub font_type: Box<dyn FontFamily>,
    pub filler: char,
}

impl StarsciiConfig {
    pub fn new() -> StarsciiConfig {
        StarsciiConfig {
            font_size: 1,
            font_type: Box::new(BoxFont {}),
            filler: '*',
        }
    }

    pub fn font_rows(&self) -> u8 {
        2 * self.font_size + 1
    }

    pub fn font_cols(&self) -> u8 {
        2 * self.font_size + 1
    }
}

pub struct Starscii {
    config: StarsciiConfig,
}

impl Starscii {
    pub fn new(config: StarsciiConfig) -> Starscii {
        Starscii { config }
    }

    pub fn draw(&self, t: &str) -> String {
        let mut conf = FontConfig {
            font_size: self.config.font_size,
            font_cols: self.config.font_size,
            font_rows: self.config.font_size,
            filler: self.config.filler,
        };
        self.config.font_type.draw(&mut conf, t)
    }
}
