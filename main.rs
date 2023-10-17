mod eoms;

use eoms::orbital_equations::OrbitalToolbox;
use eoms::orbital_objects::{COES, StateVector};

use std::f64::consts::PI;

fn main() {
    
    let mu = 398600.0;
    let radius = 6371.0;

    let ot = OrbitalToolbox::build_toolbox(mu, radius);

    let coes = COES::new(51400.0,0.0006387, 15.0 * PI / 180.0, 51.65 * PI / 180.0, 157.0 * PI / 180.0, 15.0 * PI / 180.0);

    let state = ot.coes_to_state(&coes);

    let r = state.v.to_string();

    println!("{r}");

}
