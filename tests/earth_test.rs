// tests/earth_test.rs

extern crate apollo;
use crate::apollo::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coordinate::GeographPoint;
    use crate::time;

    #[test]
    fn test_flattening_factor() {
        assert!((flattening_factor() - 1.0 / 298.257223563).abs() < 1e-10);
    }

    #[test]
    fn test_equatorial_radius() {
        assert!((equatorial_radius() - 6378.137).abs() < 1e-3);
    }

    #[test]
    fn test_polar_radius() {
        let expected = equatorial_radius() * (1.0 - flattening_factor());
        assert!((polar_radius() - expected).abs() < 1e-3);
    }

    #[test]
    fn test_eccentricity_of_meridian() {
        let f = flattening_factor();
        let expected = (f * (2.0 - f).sqrt());
        assert!((eccentricity_of_meridian() - expected).abs() < 1e-10);
    }

    #[test]
    fn test_approximate_geodesic_distance() {
        let p1 = GeographPoint::new(0.0, 0.0);
        let p2 = GeographPoint::new(1.0, 1.0);
        let distance = approximate_geodesic_distance(&p1, &p2);
        // Approximate value should be adjusted based on your test points
        assert!((distance - 111.194926).abs() < 1e-2);
    }

    #[test]
    fn test_geodesic_distance() {
        let p1 = GeographPoint::new(0.0, 0.0);
        let p2 = GeographPoint::new(1.0, 1.0);
        let distance = geodesic_distance(&p1, &p2);
        // Adjust expected value based on your specific test
        assert!((distance - 111.194926).abs() < 1e-2);
    }

    #[test]
    fn test_rho_sin_cos_phi() {
        let (rho_sin_phi, rho_cos_phi) = rho_sin_cos_phi(0.1, 1000.0);
        // Adjust expected values based on your calculations
        assert!((rho_sin_phi - 0.1).abs() < 1e-2);
        assert!((rho_cos_phi - 1.0).abs() < 1e-2);
    }

    #[test]
    fn test_distance_from_center() {
        let lat = 0.0; // Example latitude
        let expected = 0.9983271 + 0.0016764 * (2.0 * lat).cos() - 0.0000035 * (4.0 * lat).cos();
        assert!((distance_from_center(lat) - expected).abs() < 1e-7);
    }

    #[test]
    fn test_rotational_angular_velocity() {
        assert!((rotational_angular_velocity() - 0.00007292114992).abs() < 1e-14);
    }

    #[test]
    fn test_radius_of_parallel() {
        let lat = 0.1; // Example latitude
        let expected = equatorial_radius() * lat.cos()
            / (1.0 - (eccentricity_of_meridian() * lat.sin()).powi(2)).sqrt();
        assert!((radius_of_parallel(lat) - expected).abs() < 1e-2);
    }

    #[test]
    fn test_linear_velocity_at_lat() {
        let lat = 0.1; // Example latitude
        let expected = rotational_angular_velocity() * radius_of_parallel(lat);
        assert!((linear_velocity_at_lat(lat) - expected).abs() < 1e-2);
    }

    #[test]
    fn test_radius_of_curvature() {
        let lat = 0.1; // Example latitude
        let e = eccentricity_of_meridian();
        let expected =
            equatorial_radius() * (1.0 - e * e) / (1.0 - (e * lat.sin()).powi(2)).powf(1.5);
        assert!((radius_of_curvature(lat) - expected).abs() < 1e-2);
    }

    #[test]
    fn test_geograph_geocent_lat_diff() {
        let lat = 0.1; // Example latitude
        let expected = angle::deg_dmas(0, 0, 692.73) * (2.0 * lat).sin()
            - angle::deg_dmas(0, 0, 1.16) * (4.0 * lat).sin();
        assert!((geograph_geocent_lat_diff(lat) - expected).abs() < 1e-2);
    }

    #[test]
    fn test_equation_of_time() {
        let jd = 2451545.0; // Example Julian Day
        let sun_asc = 0.0; // Example right ascension of the Sun
        let nut_long = 0.0; // Example nutation correction for longitude
        let true_obliquity = 0.0; // Example true obliquity of the ecliptic
        let expected = 0.0; // Example expected result
        assert!((equation_of_time(jd, sun_asc, nut_long, true_obliquity) - expected).abs() < 1e-2);
    }

    #[test]
    fn test_angle_between_diurnal_path_and_horizon() {
        let dec = 0.1; // Example declination
        let observer_lat = 0.1; // Example observer's latitude
        let expected = (1.0 - (dec.tan() * observer_lat.tan()).powi(2))
            .sqrt()
            .atan();
        assert!(
            (angle_between_diurnal_path_and_horizon(dec, observer_lat) - expected).abs() < 1e-2
        );
    }
}
