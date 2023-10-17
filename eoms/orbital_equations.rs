// internal imports
use super::orbital_objects::{COES, StateVector};
use super::helper_functions::rotation_matrix;

// external imports
use ode_solvers::*;
use nalgebra::Vector3;

pub struct OrbitalToolbox {
    mu: f64,
    radius: f64,
    i: Vector3<f64>,
    j: Vector3<f64>,
    k: Vector3<f64>,

}

impl OrbitalToolbox {

    pub fn build_toolbox(mut mu: f64, mut radius: f64) -> OrbitalToolbox {
        OrbitalToolbox {
            mu,
            radius,
            i: Vector3::new(1.0, 0.0, 0.0),
            j: Vector3::new(0.0, 1.0, 0.0),
            k: Vector3::new(0.0, 0.0, 1.0),
        }
    }

    pub fn coes_to_state(&self, coes: &COES) -> StateVector {
        // computes state vectors from classical orbital elements
        
        let rp = (coes.h*coes.h / self.mu)*(1.0/(1.0+coes.ecc*coes.ta.cos())) * (coes.ta.cos() * self.i + coes.ta.sin() * self.j);
        let vp = (self.mu/coes.h) * (-coes.ta.sin()*self.i + ((coes.ecc + coes.ta.cos())*self.j));

        let C = rotation_matrix(coes.w, coes.inc, coes.raan, &[3,1,3]).transpose();

        let r = C * rp;
        let v = C * vp;


        StateVector::from_vectors(r, v)
        
    }
}
