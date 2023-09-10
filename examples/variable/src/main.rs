use match_to_str::match_to_str;
use rand::Rng;

#[rustfmt::skip]
mod nakano {
    pub const ICHIKA:  u8 = 1;
    pub const NINO:    u8 = 2;
    pub const MIKU:    u8 = 3;
    pub const YOTSUBA: u8 = 4;
    pub const ITSUKI:  u8 = 5;
}

fn main() {
    use nakano::*;

    let mut rng = rand::thread_rng();
    let bride = match_to_str!(rng.gen_range(1..=5) => {
        ICHIKA,
        NINO,
        MIKU,
        YOTSUBA,
        ITSUKI,
        _,
    });
    println!("{bride}");
}
