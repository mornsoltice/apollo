/*
Copyright (c) 2024 Khairandra Muhamad Nandyka

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

// !--------BINARY STAR---------!

use crate::angle;

/*
Computes mean annual motion of companion star

* `P`: Period of revolution of binary star
       (*mean solar year*)
*/

#[inline]
pub fn mean_annual_motion(P: f64) -> f64 {
    angle::TWO_PI / P
}

/*
Computes mean anomaly of companion star

# Arguments

* `n`: Mean annual motion of companion star
* `t`: Current time, given as year with
       decimals (eg: 1945.62)
* `T`: Time of periastron passage, given as
       a year with decimals (eg: 1945.62)
*/

pub fn mean_annomaly(n: f64, t: f64, T: f64) -> f64 {
    n * (t - T)
}

/// Computes the radius vector of a binary star.
///
/// # Arguments
///
/// * a - Apparent semimajor axis
/// * e - Eccentricity of the true orbit
/// * ecc_anom - Eccentric anomaly of the binary star
///
/// # Returns
///
/// The radius vector of the binary star.

pub fn radius_vector(a: f64, e: f64, ecc_anom: f64) -> f64 {
    a * (1.0 - e * ecc_anom.cos())
}

/// Computes the true anomaly of a binary star.
///
/// # Arguments
///
/// * e - Eccentricity of the true orbit
/// * ecc_anom - Eccentric anomaly of the binary star
///
/// # Returns
///
/// The true anomaly of the binary star.
#[inline]
pub fn true_anomaly(e: f64, ecc_anom: f64) -> f64 {
    2.0 * (((1.0 + e) / (1.0 - e)).sqrt() * (ecc_anom / 2.0).tan()).atan()
}

/// Computes the apparent position angle of a binary star.
///
/// # Arguments
///
/// * asc_node_coords - Position angle of ascending node
/// * true_anom - True anomaly of the binary star
/// * w - Longitude of periastron
/// * i - Inclination of the true orbit to a plane at right angles to the line of sight (in radians)
///
/// # Returns
///
/// The apparent position angle of the binary star.
pub fn apparent_position_angle(asc_node_coords: f64, true_anom: f64, w: f64, i: f64) -> f64 {
    let angle = (true_anom + w).sin() * i.cos();
    let x = angle.atan2((true_anom + w).cos());

    angle::limit_twoPI(x + asc_node_coords)
}

/// Computes the angular separation of a binary star.
///
/// # Arguments
///
/// * rad_vec - Radius vector of the binary star
/// * true_anom - True anomaly of the binary star
/// * w - Longitude of periastron
/// * i - Inclination of the true orbit to a plane at right angles to the line of sight (in radians)
///
/// # Returns
///
/// The angular separation of the binary star.
pub fn angular_separation(rad_vec: f64, true_anom: f64, w: f64, i: f64) -> f64 {
    rad_vec * (((true_anom + w).sin() * i.cos()).powi(2) + (true_anom + w).cos().powi(2)).sqrt()
}

/// Computes the eccentricity of an apparent orbit.
///
/// # Arguments
///
/// * e - Eccentricity of the true orbit
/// * w - Longitude of periastron
/// * i - Inclination of the true orbit to a plane at right angles to the line of sight (in radians)
///
/// # Returns
///
/// The eccentricity of the apparent orbit.
pub fn eccentricity_of_apparent_orbit(e: f64, w: f64, i: f64) -> f64 {
    let i_cos = i.cos();
    let e_w_cos = e * w.cos();
    let e_w_cos_squared = e_w_cos * e_w_cos;

    let a = (1.0 - e_w_cos_squared) * i_cos * i_cos;
    let b = e * w.sin() * e_w_cos * i_cos;
    let c = 1.0 - e_w_cos_squared;
    let d = ((a - c).powi(2) + 4.0 * b.powi(2)).sqrt();

    ((2.0 * d) / (a + c + d)).sqrt()
}
