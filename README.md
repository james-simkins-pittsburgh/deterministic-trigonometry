# deterministic-trigonometry
 
This library does basic trigonometry calculations without using any floating-point arithmetic.

This determinism is intended to be useful for games that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using floating point arithmetic across different architectures. This library avoids that problem by using only integer data types internally. However, this comes at the cost of imprecision from rounding errors.

Trigonometry is accomplished by using pre-baked tables of trigonometry results that are written into the code itself. It support sine, cosine, tangent, arcsine, arccosine, and arctangent.

Input is a (i32, i32) tuple representing the numerator and denominator of a fraction in radians for sine, cosine, and tangent or as a plain fraction for arcsine, arccosine, and arctangent. Output is returned as a numerator and denominator tuple (i32,i32). The output denominator is always 1000 to allow easy conversion to fixed point decimals.

# Panics

 - Denominator inputs of 0 panic as this is undefined.
 - Arcsine inputs below -1 and above 1 panic as this is undefined.
 - Arccosine inputs below -1 and above 1 panic as this is undefined.
 - If it is important that your code handles these errors gracefully, this is should be implemented in your code.

# Note on Accuracy for Sine, Cosine, and Tangent

 - For inputs with 1000 (or a factor of 1000) as the denominator and a value between 0/1000 and 6283/1000 (0 and 2 PI) the fractional result is always accurate to the nearest thousandth.
 - For inputs that are fractions with values above 6283/1000, negative fraction, and/or fractions with denominators that are not factor of 2, the results are usually accurate to the nearest thousandth but may sometimes differ by up to 1/1000 in either direction because of double rounding.
 - Much bigger differences occur when double rounding is combined with values very close to the asymptote of tangent at multiples of PI 
 away from PI/2. This is because small rounding error are amplified by the behavior of the tangent function approaching positive or negative infinity. This is not a problem between 0 and 2 PI. 
 - When in doubt, check the unit tests for each function to verify their accuracy or construct your own tests.

# Note on Accuracy for Arcsine, Arccosine, and Arctangent
 
 - For arcsine and arccosine inputs with 1000 (or a factor of 1000) in the denominator and a value between -1000/1000 and 1000/1000 (-1 and 1) the fractional result is always accurate to the nearest thousandth.
 - For arcsine and arccosine with inputs between -0.9 and 0.9 with a denominator that is not a factor of 1000 the result may differ up to 2/1000 in either direction with the error increasing on the nearer to -1 and 1 where the graph of those functions gets steeper (which magnifies rounding errors). The error gets even bigger from -1 to -0.9 and 0.9 to 1.
 - For arctangent between -4000/1000 and 4000/1000 (-4 and 4) with a denominator that is a factor of 1000 the result may differ by
 up to 1/1000 in either direction. Otherwise, the result may differ by 2/1000 in either direction. 
 - See the note below for arcsine and arccosine below -1 or above 1.