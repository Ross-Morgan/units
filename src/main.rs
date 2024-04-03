use units::unit_generic;

unit_generic!(Joule, "Joule", "J");
unit_generic!(Mole, "Mole", "mol");

fn main() {
    let energy = Joule(100.0);
    let moles = Mole(0.1);

    let energy_per_mole = energy / moles;

    println!("{}", energy_per_mole.to_string());
}
