/* This is the main struct of the library. The data holds a bunch of tables of pre-calculated trig results. The precalculation
avoids that potential problem of different results across architectures. */
pub struct DTrig {
    initialized: bool,
    sine_array: [i16; 6283],
    cosine_array: [i16; 6283],
    tangent_array: [i32; 6283],
    arcsine_array: [i16; 2001],
    arccosine_array: [i16; 2001],
    /* Size of artangent arrays is set based on the minimum spacing needed to give
    thousandths place accuracy. */
    arctangent_thousandths: [i16; 8001],
    arctangent_hundreths: [i16; 4001],
    arctangent_tenth: [i16; 2001],
    arctangent_ones: [i16; 2001],
}

// This module contains the code that sets the values for the struct.
pub mod initialize;

// This module contains utility functions.
pub mod utility;

impl DTrig {
    fn sine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            i32::from(
                self.sine_array
                    [utility::normalize_angle(utility::denominator_to_1000(argument_fraction))]
            ),
            1000,
        );
    }

    fn cosine(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            i32::from(
                self.cosine_array
                    [utility::normalize_angle(utility::denominator_to_1000(argument_fraction))]
            ),
            1000,
        );
    }

    fn tangent(&self, argument_fraction: (i32, i32)) -> (i32, i32) {
        return (
            i32::from(
                self.tangent_array
                    [utility::normalize_angle(utility::denominator_to_1000(argument_fraction))]
            ),
            1000,
        );
    }
}
