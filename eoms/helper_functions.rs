// helper functions for the program
use nalgebra::{Vector3, Matrix3};

pub fn rotation_matrix(alpha: f64, beta: f64, theta: f64, sequence: &[i64; 3]) -> Matrix3<f64> {
    let mut C:Matrix3::<f64> = Matrix3::<f64>::zeros();;

    for (i, rotation) in sequence.iter().rev().enumerate(){

        let mut angle: f64;
        let mut temp_c: Matrix3<f64>;

        // determine which angle is being used
        if i == 0 {
            angle = alpha;
        } else if i == 1 {
            angle = beta;
        } else if i == 2 {
            angle = theta;
        } else {
            panic!("Out of range");
        }

        if *rotation == 1 {
            temp_c = x_rotation(angle);
        } else if *rotation == 2 {
            temp_c = y_rotation(angle)
        } else if *rotation == 3 {
            temp_c = z_rotation(angle);
        } else {
            panic!("Rotations not in range");
        }

        if i == 0 {
            C = temp_c;
        } else {
            C *= temp_c;
        }
    }

    return C
}

pub fn x_rotation(theta: f64) -> Matrix3<f64> {
    Matrix3::new(1.0,0.0,0.0,
                0.0,theta.cos(),theta.sin(),
                0.0,-theta.sin(), theta.cos())
}

pub fn y_rotation(theta: f64) -> Matrix3<f64> {
    Matrix3::new(theta.cos(), 0.0, -theta.sin(),
                0.0, 1.0, 0.0,
                theta.sin(), 0.0, theta.cos())
}

pub fn z_rotation(theta: f64) -> Matrix3<f64> {
    Matrix3::new(theta.cos(), theta.sin(), 0.0,
            -theta.sin(), theta.cos(), 0.0,
                0.0, 0.0, 1.0)
}