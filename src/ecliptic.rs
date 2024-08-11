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


use crate::angle;
use std::f64::consts:PI;
use crate::time;

/*
Computes the mean obliquity of the ecliptic using
J. Laskar's formula

# Returns

* `mn_oblq`: Mean obliquity of the ecliptic *| in radians*

The accuracy of `mn_oblq` is estimated to be 0.01 arcseconds
for 1000 years before and after 2000 AD, and a few arcseconds
for 10000 years before and after 2000 AD.

# Arguments

* `JD`: Julian (Ephemeris) day
*/

pub fn mean_obliquity_laskar(JD: f64) -> f64 {
    let u = time::julian_century(JD);
    Horner_eval!(
        u,
        angle::deg_dmas(23, 26, 21.448),
       -angle::deg_dmas(0,  0,  4680.93),
       -angle::deg_dmas(0,  0,  1.55),
        angle::deg_dmas(0,  0,  1999.25),
       -angle::deg_dmas(0,  0,  51.38),
       -angle::deg_dmas(0,  0,  249.67),
       -angle::deg_dms(0,  0,  39.05),
        angle::deg_dmas(0,  0,  7.12),
        angle::deg_dmas(0,  0,  27.87),
        angle::deg_dmas(0,  0,  5.79),
        angle::deg_dmas(0,  0,  2.45)
    ).to_radians()
}

/**
Computes the mean obliquity of the ecliptic using
the IAU formula

# Returns

* `mn_oblq`: Mean obliquity of the ecliptic *| in radians*

The error in `mn_oblq` reaches 1 arcsecond over a period of
2000 years from 2000 AD, and about 10 arcseconds over a period of
4000 years from 2000 AD.

# Arguments

* `JD`: Julian (Ephemeris) day
**/

pub fn mean_obliquity_IAU(JD: f64) -> f64 {
    let u = time::julian_century(JD);
}



