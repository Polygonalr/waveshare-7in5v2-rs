use waveshare_7in5v2_rs::Epd;

fn main() {
    println!("Hello, world!");
    let mut e = Epd::new();
    e.clear();
}