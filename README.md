# deterministic-trigonometry
 A library for doing basic trignometry calculations without using any floating point arithmetic. 

 This library, once complete, will mainly be useful for doing trignometry for simulations within games
 that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using
 floating point arithmetic across different architextures. This library will avoid that problem by
 using only i32 integers and representing decimals as fractions.

 Trignometry will be accomplished by having pre-made tables of trignometry results held in tables that
 are hardcoded into the code itself. 

 Results are given to the nearest thousandths as integers over 1000. These are precise so long as the input
 fraction has 1, 10, 100 or 1000 or other factors of 1000 (such as 2 or 4) in the denominator. With other 
 denominators the result is rounded twice which introduces the possibility of small rounding errors.

 This library will be useful to someone making a simulation in which a small loss of precision is worth having
 exactly repoducible results across different architectures. For now, it will support sine, cosine, tangent,
 arcsine, arcosine, and arctanget.

 Input/ouput are in radians by default. All functions accept a tuple of i32 integers with the numerator first
 and the denominator second as the argument.

 