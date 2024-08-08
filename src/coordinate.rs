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

// !--------Transfer Coordinate---------!

use crate::angle;

/// Represents a point on the geographical surface of the Earth
#[derive(Debug)]
pub struct GeographPoint {
    /// Geographical longitude
    pub long: f64,
    /// Geographical latitude
    pub lat: f64,
}

impl GeographPoint {
    pub fn angular_sep(&self, other_point: &GeographPoint) -> f64 {
        angle::angular_sep(self.long, self.lat, other_point.long, other_point.lat)
    }
}

/// Represents a point in the equatorial coordinate system
#[derive(Debug)]
pub struct EqPoint {
    /// Right ascension
    pub asc: f64,
    /// Declination
    pub dec: f64,
}

impl EqPoint {
    pub fn angular_sep(&self, other_point: &EqPoint) -> f64 {
        angle::angular_sep(self.asc, self.dec, other_point.asc, other_point.dec)
    }
}

/// Represents a point in the ecliptic coordinate system
#[derive(Debug)]
pub struct EclPoint {
    /// Ecliptic longitude
    pub long: f64,
    /// Ecliptic latitude
    pub lat: f64,
}

impl EclPoint {
    pub fn angular_sep(&self, other_point: &EclPoint) -> f64 {
        angle::angular_sep(self.long, self.lat, other_point.long, other_point.lat)
    }
}

/**
Computes the hour angle from geographical longitude and Greenwich
sidereal time

# Returns

* hour_angle: Hour angle | in radians

# Arguments

* green_sidreal: Sidereal time at Greenwich | in radians
* observer_long: Observer's geographical longitude | in radians
* asc: Right ascension | in radians
**/
#[inline]
pub fn hour_angle_from_long(green_sidereal: f64, observer_long: f64, asc: f64) -> f64 {
    green_sidereal - observer_long - asc
}

/**
Computes the hour angle from local sidereal time and right
ascension

# Returns

* hour_angle: Hour angle | in radians

# Arguments

* local_sidreal: Local sidereal time | in radians
* asc: Right ascension | in radians
**/
#[inline]
pub fn hour_angle_from_sidereal(local_sidereal: f64, asc: f64) -> f64 {
    local_sidereal - asc
}

/**
Computes the ecliptic longitude from equatorial coordinates

# Returns

* ecl_long: Ecliptic longitude | in radians

# Arguments

* asc: Right ascension | in radians
* dec: Declination | in radians
* oblq_eclip: If asc and dec are corrected for
                    nutation, then true obliquity. If not, then
                    mean obliquity. | in radians
**/
pub fn ecliptic_long_from_eq(asc: f64, dec: f64, oblq_eclip: f64) -> f64 {
    (asc.sin() * oblq_eclip.cos() + dec.tan() * oblq_eclip.sin()).atan2(asc.cos())
}

/**
Computes the ecliptic latitude from equatorial coordinates

# Returns

* ecl_lat: Ecliptic latitude | in radians

# Arguments

* asc: Right ascension | in radians
* dec: Declination | in radians
* oblq_eclip: If asc and dec are corrected for
                    nutation, then true obliquity. If not, then
                    mean obliquity. | in radians
**/
pub fn ecliptic_lat_from_eq(asc: f64, dec: f64, oblq_eclip: f64) -> f64 {
    (dec.sin() * oblq_eclip.cos() - dec.cos() * oblq_eclip.sin() * asc.sin()).asin()
}

/**
Computes ecliptic coordinates from equatorial coordinates

# Returns

(ecl_long, ecl_lat)

* ecl_long: Ecliptic longitude | in radians
* ecl_lat: Ecliptic latitude | in radians

# Arguments

* $asc: Right ascension | in radians
* $dec: Declination | in radians
* $oblq_eclip: If $asc and $dec are corrected for
                     nutation, then true obliquity. If not, then
                     mean obliquity. | in radians
**/
#[macro_export]
macro_rules! ecliptic_from_eq {
    ($asc: expr, $dec: expr, $oblq_eclip: expr) => {{
        (
            apollo::coordinate::ecliptic_long_from_eq($asc, $dec, $oblq_eclip),
            apollo::coordinate::ecliptic_lat_from_eq($asc, $dec, $oblq_eclip),
        )
    }};
}

/**
Computes the right ascension from ecliptic coordinates

# Returns

* asc: Right ascension | in radians

# Arguments

* ecl_long: Ecliptic longitude | in radians
* ecl_lat: Ecliptic latitude | in radians
* oblq_eclip: If ecl_long and ecl_lat are corrected
                    for nutation, then true obliquity. If not, then
                    mean obliquity. | in radians
**/
pub fn asc_from_ecliptic(ecl_long: f64, ecl_lat: f64, oblq_eclip: f64) -> f64 {
    (ecl_long.sin() * oblq_eclip.cos() - ecl_lat.tan() * oblq_eclip.sin()).atan2(ecl_long.cos())
}

/**
Computes the declination from ecliptic coordinates

# Returns

* dec: Declination | in radians

# Arguments

* ecl_long: Ecliptic longitude | in radians
* ecl_lat: Ecliptic latitude | in radians
* oblq_eclip: If ecl_long and ecl_lat are corrected
                    for nutation, then true obliquity. If not, then
                    mean obliquity. | in radians
**/
pub fn dec_from_ecliptic(ecl_long: f64, ecl_lat: f64, oblq_eclip: f64) -> f64 {
    (ecl_lat.sin() * oblq_eclip.cos() + ecl_lat.cos() * oblq_eclip.sin() * ecl_long.sin()).asin()
}

/**
Computes equatorial coordinates from ecliptic coordinates

# Returns

(asc, dec)

* asc: Right ascension | in radians
* dec: Declination | in radians

# Arguments

* $ecl_long: Ecliptic longitude | in radians
* $ecl_lat: Ecliptic latitude | in radians
* $oblq_eclip: If $ecl_long and $ecl_lat are corrected for
                     nutation, then true obliquity. If not, then
                     mean obliquity. | in radians
**/
#[macro_export]
macro_rules! eq_from_ecliptic {
    ($ecl_long: expr, $ecl_lat: expr, $oblq_eclip: expr) => {{
        (
            apollo::coordinate::asc_from_ecliptic($ecl_long, $ecl_lat, $oblq_eclip),
            apollo::coordinate::dec_from_ecliptic($ecl_long, $ecl_lat, $oblq_eclip),
        )
    }};
}

/**
Computes the azimuth from equatorial coordinates

# Returns

* az: Azimuth | in radians

# Arguments

* hour_angle: Hour angle | in radians
* dec: Declination | in radians
* observer_lat: Observer's geographical latitude | in radians
**/
pub fn azimuth_from_eq(hour_angle: f64, dec: f64, observer_lat: f64) -> f64 {
    hour_angle
        .sin()
        .atan2(hour_angle.cos() * observer_lat.sin() - dec.tan() * observer_lat.cos())
}

/**
Computes the altitude from equatorial coordinates

# Returns

* alt: Altitude | in radians

# Arguments

* hour_angle: Hour angle | in radians
* dec: Declination | in radians
* observer_lat: Observer's geographical latitude | in radians
**/
pub fn altitude_from_eq(hour_angle: f64, dec: f64, observer_lat: f64) -> f64 {
    (observer_lat.sin() * dec.sin() + observer_lat.cos() * dec.cos() * hour_angle.cos()).asin()
}

/**
Computes local horizontal coordinates from equatorial coordinates

# Returns

(az, alt)

* az: Azimuth | in radians
* alt: Altitude | in radians

# Arguments

* $hour_angle: Hour angle | in radians
* $dec: Declination | in radians
* $observer_lat: Observer's geographical latitude | in radians
**/
#[macro_export]
macro_rules! horizontal_from_eq {
    ($hour_angle: expr, $dec: expr, $observer_lat: expr) => {{
        (
            apollo::coordinate::azimuth_from_eq($hour_angle, $dec, $observer_lat),
            apollo::coordinate::altitude_from_eq($hour_angle, $dec, $observer_lat),
        )
    }};
}

/**
Computes the hour angle from local horizontal coordinates

# Returns

* hour_angle: Hour angle | in radians

# Arguments

* az: Azimuth | in radians
* alt: Altitude | in radians
* observer_lat: Observer's geographical latitude | in radians
**/
pub fn hour_angle_from_horizontal(az: f64, alt: f64, observer_lat: f64) -> f64 {
    az.sin()
        .atan2(az.cos() * observer_lat.sin() + alt.tan() * observer_lat.cos())
}

/**
Computes the declination from local horizontal coordinates

# Returns

* dec: Declination | in radians

# Arguments

* az: Azimuth | in radians
* alt: Altitude | in radians
* observer_lat: Observer's geographical latitude | in radians
**/
pub fn dec_from_horizontal(az: f64, alt: f64, observer_lat: f64) -> f64 {
    (observer_lat.sin() * alt.sin() - observer_lat.cos() * az.cos() * az.cos()).asin()
}

/**
Computes the galactic longitude from equatorial coordinates

# Returns

* gal_long: Galactic longitude | in radians

# Arguments

* asc: Right ascension | in radians
* dec: Declination | in radians

The equatorial coordinates passed are assumed to be referred to the
standard equinox of B1950.0.
**/
pub fn galactic_long_from_eq(asc: f64, dec: f64) -> f64 {
    303_f64.to_radians()
        - (192.25_f64.to_radians() - asc).sin().atan2(
            27.4_f64.to_radians().sin() * (192.25_f64.to_radians() - asc).cos()
                - 27.4_f64.to_radians().cos() * dec.tan(),
        )
}

/**
Computes the galactic latitude from equatorial coordinates

# Returns

* gal_lat: Galactic latitude | in radians

# Arguments

* asc: Right ascension | in radians
* dec: Declination | in radians

The equatorial coordinates passed are assumed to be referred to the
standard equinox of B1950.0.
**/
pub fn galactic_lat_from_eq(asc: f64, dec: f64) -> f64 {
    (dec.sin() * 27.4_f64.to_radians().sin()
        + dec.cos() * 27.4_f64.to_radians().cos() * (192.25_f64.to_radians() - asc).cos())
    .asin()
}

/**
Computes galactic coordinates from equatorial coordinates

# Returns

(gal_long, gal_lat)

* gal_long: Galactic longitude | in radians
* gal_lat: Galactic latitude | in radians

# Arguments

* $asc: Right ascension | in radians
* $dec: Declination | in radians

The equatorial coordinates passed are assumed to be referred to the
standard equinox of B1950.0.
**/
#[macro_export]
macro_rules! galactic_from_eq {
    ($asc: expr, $dec: expr) => {{
        (
            apollo::coordinate::galactic_long_from_eq($asc, $dec),
            apollo::coordinate::galactic_lat_from_eq($asc, $dec),
        )
    }};
}

/**
Computes the right ascension from galactic coordinates

# Returns

* asc: Right ascension | in radians

The right ascension returned here is referred to the standard equinox
of  B1950.0.

# Arguments

* gal_long: Galactic longitude | in radians
* gal_lat: Galactic latitude | in radians
**/
pub fn asc_from_galactic(gal_long: f64, gal_lat: f64) -> f64 {
    12.25_f64.to_radians()
        + (gal_long - 123_f64.to_radians()).sin().atan2(
            27.4_f64.to_radians().sin() * (gal_long - 123_f64.to_radians()).cos()
                - 27.4_f64.to_radians().cos() * gal_lat.tan(),
        )
}

/*
Computes the declination from galactic coordinates

# Returns

* dec: Declination | in radians

The declination returned here is referred to the standard equinox
of  B1950.0.

# Arguments

* gal_long: Galactic longitude | in radians
* gal_lat: Galactic latitude | in radians
*/
pub fn dec_from_galactic(gal_long: f64, gal_lat: f64) -> f64 {
    (gal_lat.sin() * 27.4_f64.to_radians().sin()
        + gal_lat.cos() * 27.4_f64.to_radians().cos() * (gal_long - 123_f64.to_radians()).cos())
    .asin()
}

/*
Computes equatorial coordinates from galactic coordinates

# Returns

(asc, dec)

* asc: Right ascension | in radians
* dec: Declination | in radians

The equatorial coordinates returned here is referred to the standard
equinox of  B1950.0.

# Arguments

* $gal_long: Galactic longitude | in radians
* $gal_lat: Galactic latitude | in radians
*/
#[macro_export]
macro_rules! eq_from_galactic {
    ($gal_long: expr, $gal_lat: expr) => {{
        (
            apollo::coordinate::asc_from_galactic($gal_long, $gal_lat),
            apollo::coordinate::dec_from_galactic($gal_long, $gal_lat),
        )
    }};
}
