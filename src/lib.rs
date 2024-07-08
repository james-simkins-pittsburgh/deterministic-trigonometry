// #![warn(missing_docs)]

//! Deterministic trigonometry across architectures without using floating point arithmetic.
//!
//! - Uses (i32, i32) tuples to represent fractions.
//! - Uses pre-baked arrays for trigonometry results.
//! - Deterministic across compilers and computer architectures.
//! - Introduces imprecision due to rounding errors.
//! - Most likely to be useful for games that depend on lockstep determinism.
//!
//! # Example
//!
//! ```
//! use deterministic_trigonometry::DTrig;
//!
//! fn main (){
//!
//! let d_trig = DTrig::initialize();
//!
//! let sine_of_pi_over_three = d_trig.sine((1047,1000));
//!
//! println!("The sine of 1047/1000 radians is {}/{}.", sine_of_pi_over_three.0, sine_of_pi_over_three.1);
//!
//! }
//!
//! ```
//!

/// Main struct through which trig functions are implemented.
///
/// Once this struct is initialized, it holds arrays with pre-baked trig functions.
/// Trig functions are called as methods with the input as (i32 , i32) tuples with
/// the first i32 representing the numerator an the second i32 representing the denominator.
///
/// The output is also a (i32 , i32) tuple with the first i32 representing the numerator
/// and the second i32 representing the denominator. The denominator will always be 1000.
///
/// # Example
///
/// ```
/// use deterministic_trigonometry::DTrig;
///
/// fn main (){
///
/// let d_trig = DTrig::initialize();
///
/// let sine_of_pi_over_four = d_trig.sine((785,1000));
///
/// println!("The sine of 785/1000 radians is {}/{}.", sine_of_pi_over_four.0, sine_of_pi_over_four.1);
///
/// }
///
/// ```
///

pub struct DTrig {
    sine_array: [i16; 6283],
    cosine_array: [i16; 6283],
    tangent_array: [i32; 6283],
    arcsine_array: [i16; 2001],
    arccosine_array: [i16; 2001],
    /* Size of arctangent arrays is set based on the minimum spacing needed to give
    thousandths place accuracy. */
    arctangent_thousandths: [i16; 8001],
    arctangent_hundredths: [i16; 4001],
    arctangent_tenths: [i16; 2001],
    arctangent_ones: [i16; 2001],
}

// This module contains the code that sets the values for the arrays from the pre-baked tables.
pub mod initialize;

// This module contains utility functions.
pub mod utility;

// These functions pull the appropriate results out of the arrays.
impl DTrig {
    /// Calculates the sine of an angle in radians.
    ///
    /// - The input tuple represents the angle as a numerator and denominator.
    /// - The output tuple represents the sine as a numerator and denominator.
    /// - Most accurate between 0 and 2 PI with a factor of 1000 as denominator.
    /// - See README for limitations on accuracy.
    /// 
    /// # Example
    ///
    /// ```
    /// use deterministic_trigonometry::DTrig;
    ///
    /// fn main (){
    ///
    /// let d_trig = DTrig::initialize();
    ///
    /// let sine_of_pi_over_four = d_trig.sine((785,1000));
    ///
    /// println!("The sine of 785/1000 radians is {}/{}.", sine_of_pi_over_four.0, sine_of_pi_over_four.1);
    ///
    /// }
    ///
    /// ```

    pub fn sine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            i32::from(
                self.sine_array
                    [
                        utility::normalize_angle(
                            utility::denominator_to_1000(argument_fraction)
                        ) as usize
                    ]
            ),
            1000,
        );
    }
    /// Calculates the cosine of an angle in radians.
    ///
    /// - The input tuple represents the angle as a numerator and denominator.
    /// - The output tuple represents the sine as a numerator and denominator.
    /// - Most accurate between 0 and 2 PI with a factor of 1000 as denominator.
    /// - See README for limitations on accuracy.
    /// 
    /// # Example
    ///
    /// ```
    /// use deterministic_trigonometry::DTrig;
    ///
    /// fn main (){
    ///
    /// let d_trig = DTrig::initialize();
    ///
    /// let cosine_of_pi_over_four = d_trig.cosine((785,1000));
    ///
    /// println!("The cosine of 785/1000 radians is {}/{}.", cosine_of_pi_over_four.0, cosine_of_pi_over_four.1);
    ///
    /// }
    ///
    /// ```

    pub fn cosine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            i32::from(
                self.cosine_array
                    [
                        utility::normalize_angle(
                            utility::denominator_to_1000(argument_fraction)
                        ) as usize
                    ]
            ),
            1000,
        );
    }

    pub fn tangent(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            self.tangent_array
                [
                    utility::normalize_angle(
                        utility::denominator_to_1000(argument_fraction)
                    ) as usize
                ],
            1000,
        );
    }

    pub fn arcsine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        if utility::denominator_to_1000(argument_fraction) < -1000 {
            return (-1571, 1000);
        } else if utility::denominator_to_1000(argument_fraction) > 1000 {
            return (1571, 1000);
        } else {
            return (
                i32::from(
                    self.arcsine_array
                        [(utility::denominator_to_1000(argument_fraction) + 1000) as usize]
                ),
                1000,
            );
        }
    }

    pub fn arccosine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        if utility::denominator_to_1000(argument_fraction) < -1000 {
            return (3142, 1000);
        } else if utility::denominator_to_1000(argument_fraction) > 1000 {
            return (0, 1000);
        } else {
            return (
                i32::from(
                    self.arccosine_array
                        [(utility::denominator_to_1000(argument_fraction) + 1000) as usize]
                ),
                1000,
            );
        }
    }

    pub fn arctangent(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        // Converts the numerator to what it would be out of 1000.
        let numerator_out_of_1000 = utility::denominator_to_1000(argument_fraction);

        if numerator_out_of_1000 >= -4000 && numerator_out_of_1000 <= 4000 {
            // Handles from -4 to 4.
            return (
                i32::from(self.arctangent_thousandths[(numerator_out_of_1000 + 4000) as usize]),
                1000,
            );
        } else if numerator_out_of_1000 >= -20000 && numerator_out_of_1000 <= 20000 {
            // Handles from -20 to 20.
            if (numerator_out_of_1000 % 10).abs() < 5 {
                return (
                    i32::from(
                        self.arctangent_hundredths[(numerator_out_of_1000 / 10 + 2000) as usize]
                    ),
                    1000,
                );
            } else {
                if numerator_out_of_1000 > 0 {
                    return (
                        i32::from(
                            self.arctangent_hundredths
                                [(numerator_out_of_1000 / 10 + 1 + 2000) as usize]
                        ),
                        1000,
                    );
                } else {
                    return (
                        i32::from(
                            self.arctangent_hundredths
                                [(numerator_out_of_1000 / 10 - 1 + 2000) as usize]
                        ),
                        1000,
                    );
                }
            }
        } else if numerator_out_of_1000 >= -100000 && numerator_out_of_1000 <= 100000 {
            // Handles from -100 to 1000
            if (numerator_out_of_1000 % 100).abs() < 50 {
                return (
                    i32::from(
                        self.arctangent_tenths[(numerator_out_of_1000 / 100 + 1000) as usize]
                    ),
                    1000,
                );
            } else {
                if numerator_out_of_1000 > 0 {
                    return (
                        i32::from(
                            self.arctangent_tenths
                                [(numerator_out_of_1000 / 100 + 1 + 1000) as usize]
                        ),
                        1000,
                    );
                } else {
                    return (
                        i32::from(
                            self.arctangent_tenths
                                [(numerator_out_of_1000 / 100 - 1 + 1000) as usize]
                        ),
                        1000,
                    );
                }
            }
        } else if numerator_out_of_1000 >= -1000000 && numerator_out_of_1000 <= 1000000 {
            // Handles from -1000 to 1000.
            if numerator_out_of_1000 % 1000 < 500 {
                return (
                    i32::from(self.arctangent_ones[(numerator_out_of_1000 / 1000 + 1000) as usize]),
                    1000,
                );
            } else {
                if numerator_out_of_1000 > 0 {
                    return (
                        i32::from(
                            self.arctangent_ones[(numerator_out_of_1000 / 1000 + 1 + 1000) as usize]
                        ),
                        1000,
                    );
                } else {
                    return (
                        i32::from(
                            self.arctangent_ones[(numerator_out_of_1000 / 1000 - 1 + 1000) as usize]
                        ),
                        1000,
                    );
                }
            }
        } else {
            // Handles lower than -1000 and higher than 1000.
            if numerator_out_of_1000 < -1000000 && numerator_out_of_1000 > -3374653 {
                return (-1570, 1000);
            } else if numerator_out_of_1000 > 1000000 && numerator_out_of_1000 < 3374653 {
                return (1570, 1000);
            } else if numerator_out_of_1000 <= -3374653 {
                return (-1571, 1000);
            } else {
                return (1571, 1000);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DTrig;

    #[test]
    fn test_sine() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in 0..6283 {
            if ((((a as f64) / 1000.0).sin() * 1000.0).round() as i32) == dtrig.sine((a, 1000)).0 {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }

        for a in -1000000000..1000000001 {
            if
                (
                    ((((a as f64) / 1000.0).sin() * 1000.0).round() as i32) -
                    dtrig.sine((a, 1000)).0
                ).abs() <= 1
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_cosine() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in 0..6283 {
            if ((((a as f64) / 1000.0).cos() * 1000.0).round() as i32) == dtrig.cosine((a, 1000)).0 {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }

        for a in -1000000000..1000000001 {
            if
                (
                    ((((a as f64) / 1000.0).cos() * 1000.0).round() as i32) -
                    dtrig.cosine((a, 1000)).0
                ).abs() <= 1
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_tangent() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in 0..6283 {
            if
                ((((a as f64) / 1000.0).tan() * 1000.0).round() as i32) ==
                dtrig.tangent((a, 1000)).0
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }

        for a in -1000000000..1000000001 {
            if
                // Off by no more than .01.
                ((((a as f64) / 1000.0).tan() * 1000.0).round() as i64) -
                    (dtrig.tangent((a, 1000)).0 as i64) <= 1 ||
                // Or off by no more than 2%.
                (
                    (((a as f64) / 1000.0).tan() * 1000.0 - (dtrig.tangent((a, 1000)).0 as f64)) /
                    (((a as f64) / 1000.0).tan() * 1000.0)
                ).abs() <= 0.02 ||
                // Or if greater than 10000 off by no more than 10%.
                (((a as f64) / 1000.0).tan().abs() * 1000.0 > 10000.0 &&
                    (
                        (((a as f64) / 1000.0).tan() * 1000.0 -
                            (dtrig.tangent((a, 1000)).0 as f64)) /
                        (((a as f64) / 1000.0).tan() * 1000.0)
                    ).abs() <= 0.1) ||
                // Or if greater than 100000 just ignore it.
                (((a as f64) / 1000.0).tan() * 1000.0).abs() > 100000.0
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_arcsine() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in -1000..1001 {
            if
                ((((a as f64) / 1000.0).asin() * 1000.0).round() as i32) ==
                dtrig.arcsine((a, 1000)).0
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_arccosine() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in -1000..1001 {
            if
                ((((a as f64) / 1000.0).acos() * 1000.0).round() as i32) ==
                dtrig.arccosine((a, 1000)).0
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_arctangent() {
        let dtrig = DTrig::initialize();
        let mut result: bool;

        for a in -2000..2001 {
            if
                ((((a as f64) / 1000.0).atan() * 1000.0).round() as i32) ==
                dtrig.arctangent((a, 1000)).0
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }

        for a in -10000000..10000001 {
            if
                ((((a as f64) / 1000.0).atan() * 1000.0).round() as i32) -
                    dtrig.arctangent((a, 1000)).0 <= 1
            {
                result = true;
            } else {
                result = false;
            }

            assert_eq!(result, true);
        }

        for a in -10000..10001 {
            for b in 1..10001 {
                if
                    ((((a as f64) / (b as f64)).atan() * 1000.0).round() as i32) -
                        dtrig.arctangent((a, b)).0 <= 1
                {
                    result = true;
                } else {
                    result = false;
                }

                assert_eq!(result, true);
            }
        }
    }
}
