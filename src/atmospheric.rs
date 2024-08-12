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

// !---------ATMOSPHERIC REFRACTION----------!

use crate::angle;
use std::f64::consts::PI;

/*
Computes the refraction term for true altitudes greater than 15
degrees

# Returns

* `refrac_term`: The refraction term *| in radians*, that needs to be
                 subtracted from the apparent altitude to get the
                 true altitude

# Arguments

* `apprnt_alt`: Apparent altitude *| in radians*
*/

pub fn refrac_apparent_altitude_15(apprnt_alt: f64) -> f64 {
    let x = angle::deg_dmas(0, 0, 0.0668).to_radians() * (PI - apprnt_alt).tan();

    angle::deg_dmas(0, 0, 58.294).to_radians() * (PI - apprnt_alt).tan() - x * x * x
}
