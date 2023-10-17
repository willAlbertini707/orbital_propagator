use nalgebra::Vector3;

pub struct COES {
    // classical orbital elements
    pub h: f64, // angular momemtum km m2 / s
    pub ecc: f64, // eccentricity
    pub raan: f64, // right ascension of the ascending node (rad)
    pub inc: f64, // inclination (rad)
    pub w: f64, // argument of periapsis (rad)
    pub ta: f64, // true anomally (rad)
}

impl COES {
    pub fn new(h: f64, ecc: f64, raan: f64, inc: f64, w: f64, ta: f64) -> COES {
        COES {
            h,
            ecc,
            raan,
            inc,
            w,
            ta,
        }
    }
}

pub struct StateVector {
    pub r: Vector3<f64>, // position vector (km)
    pub v: Vector3<f64>, // velocity vector (km/s)
}

impl StateVector {
    pub fn from_vectors(r: Vector3<f64>, v: Vector3<f64>) -> StateVector {
        StateVector {
            r,
            v,
        }
    }
}