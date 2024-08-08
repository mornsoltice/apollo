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

// !-----Time For Astonomy------!

use crate::angle;

/// Represents a calendar type
#[derive(Debug)]
pub enum CalType {
    /// Gregorian calendar
    Gregorian,
    /// Julian calendar
    Julian,
}

/// Represents a month in the Gregorian and Julian calendars
#[derive(Copy, Clone, Debug)]
pub enum Month {
    /// January
    Jan = 1,
    /// February
    Feb = 2,
    /// March
    Mar = 3,
    /// April
    Apr = 4,
    /// May
    May = 5,
    /// June
    June = 6,
    /// July
    July = 7,
    /// August
    Aug = 8,
    /// September
    Sept = 9,
    /// October
    Oct = 10,
    /// November
    Nov = 11,
    /// December
    Dec = 12,
}

/// Represents a date with year, month, decimal day and calendar type
#[derive(Debug)]
pub struct Date {
    /// Year
    pub year: i16,
    /// Month
    pub month: Month,
    /// Decimal day
    ///
    /// range: 1.0 - 31.0
    pub decimal_day: f64,
    /// Calendar type
    pub cal_type: CalType,
}

/// Represents a day of a month with hours, minutes and seconds
#[derive(Debug)]
pub struct DayOfMonth {
    /// Day of month
    ///
    /// range: 1 - 31
    pub day: u8,
    /// Hour of day
    ///
    /// range: 0 - 60
    pub hr: u8,
    /// Minute of hour
    ///
    /// range: 0 - 60
    pub min: u8,
    /// Second of minute
    ///
    /// range: 0.0 - 60.0
    pub sec: f64,
    /// Time zone | in decimal hours
    ///
    /// Example: Pacific Time Zone is -8.0
    pub time_zone: f64,
}

/// Represents a day of the week
#[derive(Debug)]
pub enum Weekday {
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,
}

pub fn weekday_from_date(date: &Date) -> Weekday {
    let date_0UT = Date {
        year: date.year,
        month: date.month,
        decimal_day: date.decimal_day.floor() as f64,
        cal_type: CalType::Gregorian,
    };
    let jd = julian_day(&date_0UT);
    let wd = (jd + 1.5) as i64 % 7;

    match wd {
        0 => Weekday::Sunday,
        1 => Weekday::Monday,
        2 => Weekday::Tuesday,
        3 => Weekday::Wednesday,
        4 => Weekday::Thursday,
        5 => Weekday::Friday,
        6 => Weekday::Saturday,
        _ => panic!("Internal error in time::weekday_from_date"),
    }
}

/**
Computes decimal day for a DayOfMonth

* day_of_month: A DayOfMonth struct
**/
pub fn decimal_day(day: &DayOfMonth) -> f64 {
    (day.day as f64)
        + (day.hr as f64) / 24.0
        + (day.min as f64) / (60.0 * 24.0)
        + day.sec / (60.0 * 60.0 * 24.0)
        - day.time_zone / 24.0
}

/**
Computes decimal year for a Date

* date: A Date struct
**/
pub fn decimal_year(date: &Date) -> f64 {
    let mut y = 0;
    let mut days = 365.0;
    let month = date.month as u8;

    if month > 1 {
        y += 31;
    }
    if month > 2 {
        y += 28;
        if is_leap_year(date.year, &date.cal_type) {
            y += 1;
            days += 1.0;
        }
    }
    if month > 3 {
        y += 31;
    }
    if month > 4 {
        y += 30;
    }
    if month > 5 {
        y += 31;
    }
    if month > 6 {
        y += 30;
    }
    if month > 7 {
        y += 31;
    }
    if month > 8 {
        y += 31;
    }
    if month > 9 {
        y += 30;
    }
    if month > 10 {
        y += 31;
    }
    if month > 11 {
        y += 30;
    }

    (date.year as f64) + ((y as f64) + date.decimal_day) / days
}

/**
Checks if a year is a leap year

# Arguments

* year    : Year
* cal_type: CalType enum
**/
pub fn is_leap_year(year: i16, cal_type: &CalType) -> bool {
    match cal_type {
        CalType::Julian => year % 4 == 0,
        CalType::Gregorian => {
            if year % 100 == 0 {
                year % 400 == 0
            } else {
                year % 4 == 0
            }
        }
    }
}

/**
Computes Julian century for a Julian day

# Arguments

* jd: Julian (Ephemeris) day
**/
#[inline]
pub fn julian_century(jd: f64) -> f64 {
    (jd - 2451545.0) / 36525.0
}

/**
Computes Julian millennium for a Julian day

# Arguments

* jd: Julian (Ephemeris) day
**/
#[inline]
pub fn julian_millennium(jd: f64) -> f64 {
    (jd - 2451545.0) / 365250.0
}

/**
Computes Julian day from a Date

# Arguments

date: A Date
**/
pub fn julian_day(date: &Date) -> f64 {
    let month = date.month as u8;
    let (y, m) = if month == 1 || month == 2 {
        ((date.year - 1) as f64, (month + 12) as f64)
    } else {
        (date.year as f64, month as f64)
    };

    let a = (y / 100.0).floor();
    let b = match date.cal_type {
        CalType::Gregorian => 2.0 - a + (a / 4.0).floor(),
        CalType::Julian => 0.0,
    };

    (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + date.decimal_day + b - 1524.5
}

/**
Computes the Julian Ephemeris day

# Arguments

* jd     : Julian day
* delta_t: Delta T
**/
#[inline]
pub fn julian_ephemeris_day(jd: f64, delta_t: f64) -> f64 {
    delta_t / 86400.0 + jd
}

/**
Computes a year, month and decimal day equivalent to a given Julian day

# Returns

(year, month, decimal_day)

* year       : Year
* month      : Month
* decimal_day: Decimal day

# Arguments

jd: Julian Day. *Can't be a negative value.*
**/
pub fn date_from_julian_day<'a>(mut jd: f64) -> Result<(i16, u8, f64), &'a str> {
    if jd < 0.0 {
        return Err("A negative value for JD was passed to time::date_from_julian_day()");
    }

    jd += 0.5;
    let z = jd as i64;
    let f = jd - (z as f64);

    let a = if z < 2299161 {
        z
    } else {
        let alpha = (((z as f64) - 1867216.25) / 36524.25).floor() as i64;
        z + 1 + alpha - ((alpha as f64) / 4.0).floor() as i64
    };

    let b = a + 1524;
    let c = (((b as f64) - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * (c as f64)).floor() as i64;
    let e = (((b - d) as f64) / 30.6001).floor() as i64;

    let day = ((b - d) as f64) - (30.6001 * (e as f64)).floor() + f;

    let month = if e < 14 {
        e - 1
    } else if e == 14 || e == 15 {
        e - 13
    } else {
        return Err("Internal error in time::date_from_julian_day()");
    };

    let year = if month > 2 {
        c - 4716
    } else if month == 1 || month == 2 {
        c - 4715
    } else {
        return Err("Internal error in time::date_from_julian_day()");
    };

    Ok((year as i16, month as u8, day))
}

/**
Computes apparent sidereal time from the mean sidereal time

# Returns

* apparent_sidereal: Apparent sidereal time | in radians

# Arguments

* mean_sidereal    : Mean sidereal time | in radians
* nut_in_long: Nutation in longitude | in radians
* true_obliquity  : True obliquity of the ecliptic | in radians
**/
pub fn apparent_sidereal(mean_sidereal: f64, nut_in_long: f64, true_obliquity: f64) -> f64 {
    mean_sidereal + nut_in_long * true_obliquity.cos()
}

/**
Computes apparent sidereal time for a Julian day

This functions uses internally J. Laskar's formula for
computing the obliquity of the ecliptic.

# Returns

* apparent_sidereal: Apparent sidereal time | in radians

# Arguments

* $jd: Julian day
**/
#[macro_export]
macro_rules! apparent_sidereal {
    ($jd: expr) => {{
        let (nut_in_long, nut_in_obliquity) = apollo::nutation::nutation($jd);
        let ecliptic_obliquity = apollo::ecliptic::mean_obliquity_laskar($jd);
        apollo::time::apparent_sidereal(
            apollo::time::mean_sidereal($jd),
            nut_in_long,
            ecliptic_obliquity + nut_in_obliquity,
        )
    }};
}

/**
Computes mean sidereal time for a Julian day

# Returns

* mean_sidereal: Mean sidereal time | in radians

# Arguments

* jd: Julian day
**/
pub fn mean_sidereal(jd: f64) -> f64 {
    let jc = julian_century(jd);

    angle::limit_360(
        280.46061837
            + 360.98564736629 * (jd - 2451545.0)
            + jc * jc * (0.000387933 - jc / 38710000.0),
    )
    .to_radians()
}

/**
Computes an approximate value of ΔT for a given year and month

This function approximates ΔT from polynomial expressions using a
method different from that given in the Meeus book. The method
used is given [here](http://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html);
it covers a far wider time range, and is more accurate.

# Arguments

* year : Year
* month: Month range: 1 - 12
**/
pub fn delta_t(year: i32, month: u8) -> f64 {
    let y = (year as f64) + ((month as f64) - 0.5) / 12.0;

    if y < -500.0 {
        let u = (y - 1820.0) / 100.0;
        return 32.0 * u * u - 20.0;
    } else if y < 500.0 {
        let u = y / 100.0;
        return 10583.6
            - u * (1014.41
                + u * (33.78311
                    + u * (5.952053 - u * (0.1798452 - u * (0.022174192 - u * 0.0090316521)))));
    } else if y < 1600.0 {
        let u = (y - 1000.0) / 100.0;
        return 1574.2
            - u * (556.01
                - u * (71.23472
                    + u * (0.319781 - u * (0.8503463 + u * (0.005050998 - u * 0.0083572073)))));
    } else if y < 1700.0 {
        let u = y - 1600.0;
        return 120.0 - u * (0.9808 + u * (0.01532 - u / 7129.0));
    } else if y < 1800.0 {
        let u = y - 1700.0;
        return 8.83 + u * (0.1603 - u * (0.0059285 - u * (0.00013336 + u / 1174000.0)));
    } else if y < 1860.0 {
        let u = y - 1800.0;
        return 13.72
            - u * (0.332447
                - u * (0.0068612
                    + u * (0.0041116
                        - u * (0.00037436
                            - u * (0.0000121272 + u * (0.0000001699 - u * 0.000000000875))))));
    } else if y < 1900.0 {
        let u = y - 1860.0;
        return 7.62
            + u * (0.5737 - u * (0.251754 - u * (0.01680668 - u * (0.0004473624 + u / 233174.0))));
    } else if y < 1920.0 {
        let u = y - 1900.0;
        return -2.79 + u * (1.494119 - u * (0.0598939 - u * (0.0061966 + u * 0.000197)));
    } else if y < 1941.0 {
        let u = y - 1920.0;
        return 21.20 + u * (0.84493 - u * (0.076100 - u * 0.0020936));
    } else if y < 1961.0 {
        let u = y - 1950.0;
        return 29.07 + u * (0.407 - u * ((1.0 / 233.0) - u / 2547.0));
    } else if y < 1986.0 {
        let u = y - 1975.0;
        return 45.45 + u * (1.067 - u * ((1.0 / 260.0) + u / 718.0));
    } else if y < 2005.0 {
        let u = y - 2000.0;
        return 63.86
            + u * (0.3345
                - u * (0.060374 - u * (0.0017275 + u * (0.000651814 + u * 0.00002373599))));
    } else if y < 2050.0 {
        let u = y - 2000.0;
        return 62.92 + u * (0.32217 + u * 0.005589);
    } else if y <= 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return 32.0 * u * u - 20.0 - 0.5628 * (2150.0 - y);
    } else if y > 2150.0 {
        let u = (y - 1820.0) / 100.0;
        return 32.0 * u * u - 20.0;
    }

    0.0
}
