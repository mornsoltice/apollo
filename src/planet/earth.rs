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

// !------EARTH------!

use crate::angle;
use crate::coordinate;
use crate::time;

/*
Returns the flattening factor of the Earth

Reference: [World Geodetic System 1984](https://confluence.qps.nl/pages/viewpage.action?pageId=29855173)
*/

#[inline(always)]
pub fn flattening_factor() -> f64 {
    1.0 / 298.257223563
}

/*
Returns the equatorial radius of the Earth | in kilometers

Reference: [World Geodetic System 1984](https://confluence.qps.nl/pages/viewpage.action?pageId=29855173)
*/

#[inline(always)]
pub fn equatorial_radius() -> f64 {
    6378.137
}

/*
Returns the polar radius of the Earth | in kilometers

Calculated using [flattening_factor()](./fn.flattening_factor.html) and
[equatorial_radius()](./fn.equatorial_radius.html)
*/
#[inline]
pub fn polar_radius() -> f64 {
    equatorial_radius() * (1.0 - flattening_factor())
}

/*
Return the eccentricity of the Earth meridian
*/
#[inline]
pub fn eccentricity_of_meridian() -> f64 {
    (flattening_factor() * (2.0 - flattening_factor()).sqrt())
}

/*
Computes a low accuracy geodesic distance between two points
on the Earth's surface | in kilometers

Assumes that the Earth is a sphere.

# Arguments

* p1: GeographPoint 1
* p2: GeographPoint 2
*/

#[inline]
pub fn approximate_geodesic_distance(
    p1: &coordinate::GeographPoint,
    p2: &coordinate::GeographPoint,
) -> f64 {
    6371.0 * p1.angular_sep(&p2)
}

/**
Computes a high accuracy geodesic distance between two points on the Earth's
surface | in kilometers

# Arguments

* p1: GeographPoint 1
* p2: GeographPoint 2
**/
pub fn geodesic_distance(p1: &coordinate::GeographPoint, p2: &coordinate::GeographPoint) -> f64 {
    let f = (p1.lat + p2.lat) / 2.0;
    let g = (p1.lat - p2.lat) / 2.0;
    let lam = (p1.long - p2.long) / 2.0;

    let s = (g.sin() * lam.cos()).powi(2) + (f.cos() * lam.sin()).powi(2);
    let c = (g.cos() * lam.cos()).powi(2) + (f.sin() * lam.sin()).powi(2);
    let om = ((s / c).sqrt()).atan();

    let r = (s * c).sqrt() / om;
    let d = 2.0 * om * equatorial_radius();

    let h1 = (3.0 * r - 1.0) / (2.0 * c);
    let h2 = (3.0 * r + 1.0) / (2.0 * s);

    d * (1.0 + flattening_factor() * h1 * (f.sin() * g.cos()).powi(2)
        - flattening_factor() * h2 * (f.cos() * g.sin()).powi(2))
}

/**
Computes two quantities that are used elsewhere in the library

# Returns

Rho here denotes the distance from the Earth's center to a point
on the ellipsoid, and Phi here denotes the geocentric latitude,
both of an observer on the Earth's surface.

rho_sin_phi, rho_cos_phi

* rho_sin_phi: Rho sin phi
* rho_cos_phi: Rho cos phi

# Arguments

* geograph_lat: Observer's geographic latitude | in radians
* height      : Observer's height above sea level (meters)
**/
pub fn rho_sin_cos_phi(geograph_lat: f64, height: f64) -> (f64, f64) {
    let u = (geograph_lat.tan() * polar_radius() / equatorial_radius()).atan();
    let x = height / (equatorial_radius() * 1000.0);

    let rho_sin_phi = (u.sin() * polar_radius() / equatorial_radius()) + (geograph_lat.sin() * x);
    let rho_cos_phi = u.cos() + (geograph_lat.cos() * x);

    (rho_sin_phi, rho_cos_phi)
}

/**
Computes the distance from the Earth's center to a point on the
ellipsoid

# Returns

* rho: Distance from the Earth's center to the point on the
         ellipsoid (fraction of the equatorial radius)

# Arguments

* geograph_lat: Geographic latitude of a point on the
                  ellipsoid | in radians
**/
pub fn distance_from_center(geograph_lat: f64) -> f64 {
    0.9983271 + 0.0016764 * (2.0 * geograph_lat).cos() - 0.0000035 * (4.0 * geograph_lat).cos()
}

/// Returns the rotational angular velocity of the Earth
/// | in radian per second
#[inline(always)]
pub fn rotational_angular_velocity() -> f64 {
    0.00007292114992
}

/**
Computes the radius of the parallel of a latitude

# Returns

* rad: Radius of the parallel of the latitude
         | in kilometers

# Arguments

* geograph_lat: Geographic latitude of a point
                  on the ellipsoid | in radians
**/
pub fn radius_of_parallel(geograph_lat: f64) -> f64 {
    let e = eccentricity_of_meridian();

    equatorial_radius() * geograph_lat.cos() / (1.0 - (e * geograph_lat.sin()).powi(2)).sqrt()
}

/**
Computes the linear velocity of a point at a latitude

# Returns

* lin_vel: Linear velocity at the latitude
             (kilometers per second*)

# Arguments

* geograph_lat: Geographic latitude of a point on
                  the ellipsoid | in radians
**/
#[inline(always)]
pub fn linear_velocity_at_lat(geograph_lat: f64) -> f64 {
    rotational_angular_velocity() * radius_of_parallel(geograph_lat)
}

/**
Computes the radius of curvature of the Earth's meridian
at a latitude

# Returns

* rad: Radius of curvature of the Earth's meridian at the
         latitude | in kilometers

# Arguments

* geograph_lat: Geographic latitude of a point on the
                  ellipsoid | in radians
**/
pub fn radius_of_curvature(geograph_lat: f64) -> f64 {
    let e = eccentricity_of_meridian();

    equatorial_radius() * (1.0 - e * e) / (1.0 - (e * geograph_lat.sin()).powi(2)).powf(1.5)
}

/**
Computes the difference between the geographic latitude and
geocentric latitude

# Returns

* diff: Geographic latitude minus geocentric latitude
          | in radians

# Arguments

* geograph_lat: Geographic latitude | in radians
**/
pub fn geograph_geocent_lat_diff(geograph_lat: f64) -> f64 {
    angle::deg_dmas(0, 0, 692.73) * (2.0 * geograph_lat).sin()
        - angle::deg_dmas(0, 0, 1.16) * (4.0 * geograph_lat).sin()
}

/**
Computes the equation of time | in radians

# Arguments

* jd      : Julian (Ephemeris) day
* sun_asc : Right ascension of the Sun | in radians
* nut_long : Nutation correction for longitude | in radians
* true_obliquity: True obliquity of the ecliptic | in radians
**/
pub fn equation_of_time(jd: f64, sun_asc: f64, nut_long: f64, true_obliquity: f64) -> f64 {
    let t = time::julian_millennium(jd);
    let l = angle::limit_360(
        280.4664567
            + t * (360007.6982779
                + t * (0.030328 + t * (1.0 / 49931.0 - t * (1.0 / 15300.0 + t / 2000000.0)))),
    );

    (l - 0.0057183 - sun_asc.to_degrees() + nut_long.to_degrees() * true_obliquity.cos())
        .to_radians()
}

/**
Computes the angle between diurnal path and the horizon

# Returns

* angl: Angle between the diurnal path of a celestial
          body and the horizon | in radians

# Arguments

* dec         : Declination of the celestial body
                  | in radians
* observer_lat: Observer's geographic latitude
                  | in radians
**/
pub fn angle_between_diurnal_path_and_horizon(dec: f64, observer_lat: f64) -> f64 {
    let b = dec.tan() * observer_lat.tan();
    let c = (1.0 - b * b).sqrt();

    (c * dec.cos()).atan2(observer_lat.tan())
}
