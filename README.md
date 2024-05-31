# deterministic-trigonometry

This library is currently being written and is not in a usable state!
 
This library will do basic trigonometry calculations without using any floating-point arithmetic. 

This library, once completed, will mainly be useful for trigonometry for simulations within games that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using floating point arithmetic across different architectures. This library will avoid that problem byusing only integer data types internally.

Trigonometry will be accomplished by having pre-baked tables of trigonometry results that are written into the code itself.

This library will be useful to someone making a simulation in which a small loss of precision is worth having exactly reproducible results across different architectures. For now, it will support sine, cosine, tangent, arcsine, arccosine, and arctangent.

Input will be given as an (i32, i32) tuple representing the numerator and denominator of a fraction in radians for sine, cosine, and tangent or as a plain fraction for arcsine, arccosine, and arctangent. All results are returned as a numerator and denominator tuple (i32,i32). The output denominator is always 1000 to allow easy conversion to fixed point decimals.
 
# Note on Accuracy

 - For inputs with 1000 (or a factor of 1000) in the denominator and a value between 0/1000 and 6283/1000 (0 and 2 PI) the fractional result is always accurate to the nearest thousandth.
 - For inputs that are fractions with values above 6283/1000, negative fraction, and/or fractions with denominators that are not factor of 2, the results are usually accurate to the nearest thousandth but may sometimes differ by 1/1000 in either direction because of double rounding.
 - Much bigger differences occur when double rounding is combined with values very close to the asymptote of tangent at multiples of 
 PI/2. This is because small rounding error are amplified by the behavior of the tangent function approaching positive or negative
 infinity. This is not a problem between 0 and 2 PI. 
 - When in doubt, check the unit tests for each function to verify their accuracy or construct your own tests.

# Note on Domains of Arcsine and Arccosine

 - Arcsine inputs below -1 and above 1 return as -1571/1000 (-PI/2) and 1571/1000 (PI/2) when that is really mathematically undefined.
 - Arccosine inputs below -1 and above 1 return as 3142/1000 (PI) and 0/1000 when that is really mathematically undefined.
 - This decision made to allow for ease of use. If it is important to detect erroneous inputs outside of the true domain
 for arcsine and arccosine, this error detection should be implemented in the code using this library.
