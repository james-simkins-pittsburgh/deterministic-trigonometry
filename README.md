# deterministic-trigonometry
 This library is currently being written and is not in a usable state!
 
 It will be a library for doing basic trigonometry calculations without using any floating-point arithmetic. 

 This library, once completed, will mainly be useful for doing trigonometry for simulations within games
 that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using
 floating point arithmetic across different architectures. This library will avoid that problem by
 using only integer data types internally.

 Trigonometry will be accomplished by having pre-baked tables of trigonometry results that are written into 
 the code itself.

 This library will be useful to someone making a simulation in which a small loss of precision is worth having
 exactly reproducible results across different architectures. For now, it will support sine, cosine, tangent,
 arcsine, arccosine, and arctangent.

 Input will be given as an (i32, i32) tuple representing the numerator and denominator of a fraction in radians for
 angles or as a number for proportions. With the exception of the limitations below, all return values should be returned as a numerator and denominator tuple (i32,i32) representing a fraction which is accurate to the nearest thousanth. The second i32 will always be 1000. 
 
 Limitations:

 - Inputs with very high absolute values (both positive and negative) for sine, cosine and tangent may be slightly inaccurate due to the way in which the program rounds number when it regularizes angles.
 - Input values that use denominators that are not factors of 1000 may have slight rounding errors.
 - Tangent values very close to PI / 2 may be inaccurate due to rounding very close to the limit at PI / 2.
 - Arcsin below -1 and above 1 return as -1571/1000 (-PI/2) and 1571/1000 (PI/2) when that is really undefined.
 - Arccos below -1 and above 1 return as 3142/1000 (PI) and 0/1000 when that is really undefined.