/* This is the main struct of the library. The data holds a bunch of tables of pre-calculated trig results. The precalculation
avoids that potential problem of different results across architectures. */
pub struct DTrig {
    pub sine_array: [i16; 6283],
    pub cosine_array: [i16; 6283],
    pub tangent_array: [i16; 6283],
    pub arcsine_array: [i16; 2001],
    pub arccosine_array: [i16; 2001],
    /* Size of artangent arrays is set based on the minimum spacing needed to give
    thousandths place accuracy. */
    pub arctanent_thousandths: [i16; 4001],
    pub arctangent_hundreths: [i16; 2001],
    pub arctanent_tenth: [i16; 1001],
    pub arctangent_ones: [i16; 1001],
}