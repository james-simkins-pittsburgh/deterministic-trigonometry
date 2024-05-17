/* This is the main struct of the library. The data holds a bunch of tables of pre-calculated trig results. The precalculation
avoids that potential problem of different results across architectures. */
pub struct DTrig {
    sine_array: [i16; 6283],
    cosine_array: [i16; 6283],
    tangent_array: [i16; 6283],
    arcsine_array: [i16; 2001],
    arccosine_array: [i16; 2001],
    /* Size of artangent arrays is set based on the minimum spacing needed to give
    thousandths place accuracy. */
    arctanent_thousandths: [i16; 4001],
    arctangent_hundreths: [i16; 2001],
    arctanent_tenth: [i16; 1001],
    arctangent_ones: [i16; 1001],
}

// This module contains the code that sets the values for the struct.
pub mod initialize;


