use std::ops::Add;
struct Millimeter(u32);
struct Meter(u32);

impl Add<&Meter> for &Millimeter {
    type Output = Millimeter;

    fn add(self, other: &Meter) -> Millimeter {
        Millimeter(self.0 + (other.0 * 1000))
    }
}
fn main() {
    let milliMeter = Millimeter(1000);
    let meter = Meter(2);

    println!(
        // "{}",
        "{}Meter + {}Millimeter ={}Millimeter",
        meter.0,
        milliMeter.0,
        (&milliMeter + &meter).0
    );
}
