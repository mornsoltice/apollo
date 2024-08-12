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
use crate::coordinate;
use crate::time;

/*
Computes the equatorial horizontal parallax of the Moon

# Returns

* `eq_hz_parllx`: Equatorial horizontal parallax of the
                  Moon *| in radians*

# Arguments

* `earth_moon_dist`: Earth-Moon distance *| in kilometers*
*/

#[inline]
pub fn horizontal_parallax(earth_moon_dist: f64) -> f64 {
    (6378.14 / earth_moon_dist).asin()
}

/*
Computes the equatorial semidiameter of the Moon

# Returns

* `eq_semidiameter`: Geocentric equatorial semidiameter
                     *| in radians per kilometers*

# Arguments

* `earth_moon_dist`: Earth-Moon distance *| in kilometers*
*/

pub fn semidiameter(earth_moon_dist: f64) -> f64 {
    0.272481 * horizontal_parallax(earth_moon_dist).sin()
}
