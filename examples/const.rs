use match_to_str::match_to_str;
use rand::Rng;

const ICHIKA:  u8 =  1;
const NINO:    u8 =  2;
const MIKU:    u8 =  3;
const YOTSUBA: u8 =  4;
const ITSUKI:  u8 =  5;

fn main() {
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
