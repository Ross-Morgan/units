use units::{unit, value};

unit!(Joule, "Joule", "J");
unit!(Mole, "Mole", "mol");

fn main() {
    let energy = value!(100.0f64, Joule);
    let amount = value!(0.1f64, Mole);
    let energy_per_amount = energy / amount;

    println!("{}", energy.to_string());
    println!("{}", amount.to_string());
    println!("{}", energy_per_amount.to_string());
}
