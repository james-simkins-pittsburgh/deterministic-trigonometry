/* This is the main struct of the library. The data holds a bunch of tables of pre-calculated trig results. The precalculation
avoids that potential problem of different results across architectures. */
pub struct DTrig {
    sine_array: [i16; 6283],
    cosine_array: [i16; 6283],
    tangent_array: [i32; 6283],
    arcsine_array: [i16; 2001],
    arccosine_array: [i16; 2001],
    /* Size of artangent arrays is set based on the minimum spacing needed to give
    thousandths place accuracy. */
    arctangent_thousandths: [i16; 8001],
    arctangent_hundreths: [i16; 4001],
    arctangent_tenths: [i16; 2001],
    arctangent_ones: [i16; 2001],
}

// This module contains the code that sets the values for the arrays from the prebaked tables.
pub mod initialize;

// This module contains utility functions.
pub mod utility;

// These functions pull the appropriate results out of the arrays.
impl DTrig {
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
                        [
                            ((utility::denominator_to_1000(argument_fraction) as usize) +
                                1000) as usize
                        ]
                ),
                1000,
            );
        }
    }

    pub fn artangent(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        // Converts the numerator to what it would be out of 1000.
        let numerator_out_of_1000 = utility::denominator_to_1000(argument_fraction);

        if numerator_out_of_1000 >= -4000 && numerator_out_of_1000 <= 4000 {
            // Handles from -4 to 4.
            return (
                i32::from(self.arctangent_thousandths[(numerator_out_of_1000 + 4000) as usize]),
                1000,
            );
        } else if numerator_out_of_1000 >= -20000 && numerator_out_of_1000 <= 20000 {
            // Hangles from -20 to 20.
            if numerator_out_of_1000 % 10 < 5 {
                return (
                    i32::from(
                        self.arctangent_hundreths[(numerator_out_of_1000 / 10 + 2000) as usize]
                    ),
                    1000,
                );
            } else {
                return (
                    i32::from(
                        self.arctangent_hundreths[(numerator_out_of_1000 / 10 + 1 + 2000) as usize]
                    ),
                    1000,
                );
            }
        } else if numerator_out_of_1000 >= -100000 && numerator_out_of_1000 <= 100000 {
            // Handles from -100 to 1000
            if numerator_out_of_1000 % 100 < 50 {
                return (
                    i32::from(
                        self.arctangent_tenths[(numerator_out_of_1000 / 100 + 1000) as usize]
                    ),
                    1000,
                );
            } else {
                return (
                    i32::from(
                        self.arctangent_tenths[(numerator_out_of_1000 / 100 + 1 + 1000) as usize]
                    ),
                    1000,
                );
            }
        } else if numerator_out_of_1000 >= -1000000 && numerator_out_of_1000 <= 1000000 {
            // Handles from -1000 to 1000.
            if numerator_out_of_1000 % 1000 < 500 {
                return (
                    i32::from(self.arctangent_ones[(numerator_out_of_1000 / 1000 + 1000) as usize]),
                    1000,
                );
            } else {
                return (
                    i32::from(
                        self.arctangent_tenths[(numerator_out_of_1000 / 1000 + 1 + 1000) as usize]
                    ),
                    1000,
                );
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

        for a in 0..6283 {
            if ((((a as f64) / 1000.0).sin() * 1000.0).round() as i32) != dtrig.sine((a, 1000)).0 {
                let b = ((a as f64) / 1000.0).sin() * 1000.0;
                let c = dtrig.sine((a, 1000)).0;
                println!(" {} {} {} ", a, b, c);
            }

            assert_eq!(
                (((a as f64) / 1000.0).sin() * 1000.0).round() as i32,
                dtrig.sine((a, 1000)).0
            );
        }
    }
}
