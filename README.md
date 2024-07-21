## deterministic-trigonometry
 
This library provides basic trigonometry functions without using any floating point arithmetic.

This library is intended to be useful for games that use lockstep determinism and therefore wish to avoid the indeterminism that comes with using floating point arithmetic across different hardware or compilers. This library avoids these tiny inconsistencies by using only integer data types internally. Therefore, this library should produce exactly reproducible results regardless of the compiler or hardware used. However, this comes at the cost of imprecision because of compounded rounding errors. (Note: This imprecision is still 100% consistent and reproducible so it will not break determinism.)

Trigonometry is accomplished by using pre-baked tables of trigonometry results that are written into the code itself and then written into memory with an initialize() function. This library supports sine, cosine, tangent, arcsine, arccosine, and arctangent.

Provide input as a (i32, i32) tuple corresponding to the numerator and denominator of the input represented as a fraction. All angle measurements are in radians. Output is returned as a tuple (i32,i32) representing a fractional output. The output denominator is always 1000 to allow easy conversion to fixed point decimals.

## Adding it to Your Rust Project

```
cargo add deterministic-trigonometry
```

## Basic Example

```rust
use deterministic_trigonometry::DTrig;

fn main (){

let d_trig = DTrig::initialize();

let arctangent_of_one_half = d_trig.arctangent((500,1000));
 
println!("The arctangent of 500/1000 radians is {}/{}.", arctangent_of_one_half.0, arctangent_of_one_half.1);

}
```

For a more complex example, see the examples folder in the source code.

## List of Functions

The library has an initialize() function that must be run when the DTrig struct is instantiated
and the six trigonometry and inverse trigonometry functions:

```rust

let d_trig = DTrig::initialize();

let sine_of_one_half = d.trig.sine((500,1000));
let cosine_of_one_half = d.trig.cosine((500,1000));
let tangent_of_one_half = d.trig.tangent((500,1000));
let arcsine_of_one_half = d.trig.arcsine((500,1000));
let arccosine_of_one_half = d.trig.arccosine((500,1000));
let arctangent_of_one_half = d.trig.arctangent((500,1000));

```


## Things that Cause the Library to Panic

 - Denominator inputs of 0 panic as division by 0 is undefined.
 - Arcsine inputs below -1 and above 1 panic as this is mathematically undefined for arcsine.
 - Arccosine inputs below -1 and above 1 panic as this is mathematically undefined for arccosine.
 - If it is important that your code handles these errors gracefully, this should be implemented in your code.

## Note on Accuracy for Sine, Cosine, and Tangent

 - For inputs with 1000 (or a factor of 1000) as the denominator and a value between 0/1000 and 6283/1000 (0 and 2 PI) the fractional result is always accurate to the nearest thousandth.
 - For inputs that are fractions with values above 6283/1000, negative fractions, and/or fractions with denominators that are not a factor of 2, the results are usually accurate to the nearest thousandth but may sometimes differ by up to 1/1000 in either direction because of double rounding.
 - Much bigger differences occur when double rounding is combined with values very close to the asymptote of the tangent at multiples of PI away from PI/2 in either direction. This is because small rounding errors are amplified by the behavior of the tangent function approaching positive or negative infinity. This is not a problem between 0 and 2 PI. 
 - If accuracy is important, check the integration tests for each function to verify their accuracy or construct your own tests.

## Note on Accuracy for Arcsine, Arccosine, and Arctangent
 
 - For arcsine and arccosine inputs with 1000 (or a factor of 1000) in the denominator the fractional result is always accurate to the nearest thousandth.
 - For arcsine and arccosine with inputs between -0.9 and 0.9 and a denominator that is not a factor of 1000 the result may differ up to 2/1000 in either direction with the error increasing nearer to -0.9 and 0.9 where the graph of those functions gets steeper (which magnifies rounding errors). The error gets even bigger from -1 to -0.9 and 0.9 to 1.
 - For arctangent inputs between -4000/1000 and 4000/1000 (-4 and 4) with a denominator that is a factor of 1000 the result may differ by up to 1/1000 in either direction. Otherwise, the result may differ by up to 2/1000 in either direction. 
 - If accuracy is important, check the integration tests for each function to verify their accuracy or construct your own tests.

 ## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
