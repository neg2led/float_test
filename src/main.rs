use rand::{thread_rng, Rng};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("rust float_test v{}", VERSION);
    let mut rng = thread_rng();

    println!("Generating two random floats...");

    let x: f32 = rng.gen::<f32>();
    let y: f32 = rng.gen::<f32>();

    println!("x = {}, y = {}", x, y);
    let sum = x + y;
    let diff = x - y;
    let prod = x * y;

    println!("sum = {}, diff = {}, prod = {}", sum, diff, prod);
    println!("if this didn't crash, float is working! yay!");
}
