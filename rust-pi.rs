// pub const PI: f64 = 3.14159265358979323846264338327950288f64;

fn main() {
    let mut pi: f64 = 4.0;
    let mut div: f64 = 1.0;
    for i in 0..std::usize::MAX {
        div += 2.0;
        if i % 2 == 0 {
            pi -= 4.0/div;
        } else {
            pi += 4.0/div;
        }
        println!("approx: {},\t % err: {}", pi, 100.0 * (pi - std::f64::consts::PI).abs() / std::f64::consts::PI);
    }
}