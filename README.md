# deterministic-trigonometry
 This library is currently being written and is not in a usable state!
 
 It will be a library for doing basic trigonometry calculations without using any floating-point arithmetic. 

 This library, once completed, will mainly be useful for doing trigonometry for simulations within games
 that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using
 floating point arithmetic across different architectures. This library will avoid that problem by
 using only integer data types and representing decimals as fractions.

 Trigonometry will be accomplished by having pre-made tables of trigonometry results that are written into 
 the code itself. 

 This library will be useful to someone making a simulation in which a small loss of precision is worth having
 exactly reproducible results across different architectures. For now, it will support sine, cosine, tangent,
 arcsine, arcosine, and arctanget.
 
