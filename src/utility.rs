// Outputs the nearest integer fraction out of 1000 to the argument fraction.
pub fn denominator_to_1000(argument_fraction_i32: (i32, i32)) -> i64 {
    let argument_fraction = (
        i64::from(argument_fraction_i32.0),
        i64::from(argument_fraction_i32.1),
    );
    // No need to do anything if input is already out of 1000.
    if argument_fraction.1 == 1000 {
        return i64::from(argument_fraction.0);
    } else {
        // Handles other denominators with appropriate rounding to nearest thousandth.
        if argument_fraction.1 % 2 == 0 {
            if
                ((argument_fraction.0 * 1000) % argument_fraction.1).abs() >=
                (argument_fraction.1 / 2).abs()
            {
                if argument_fraction.0 * argument_fraction.1 >= 0 {
                    return (argument_fraction.0 * 1000) / argument_fraction.1 + 1;
                } else {
                    return (argument_fraction.0 * 1000) / argument_fraction.1 - 1;
                }
            } else {
                return (argument_fraction.0 * 1000) / argument_fraction.1;
            }
        } else {
            if
                ((argument_fraction.0 * 1000) % argument_fraction.1).abs() >
                (argument_fraction.1 / 2).abs()
            {
                if argument_fraction.0 * argument_fraction.1 >= 0 {
                    return (argument_fraction.0 * 1000) / argument_fraction.1 + 1;
                } else {
                    return (argument_fraction.0 * 1000) / argument_fraction.1 - 1;
                }
            } else {
                return (argument_fraction.0 * 1000) / argument_fraction.1;
            }
        }
    }
}

// Normalizes angles to 0 to 6282 thousandth radians

pub fn normalize_angle(angle: i64) -> i64 {
    let mut return_angle = angle;

    // Handles the case in which the angle is greater than or equal to 2 * pi thousandth radians.
    if return_angle > 6282 {
        // Multiplied by 1000000 to allow more precision.
        let mut angle_times_a_billion = i128::from(return_angle * 1000000000);

        // Normalizes angle
        angle_times_a_billion = angle_times_a_billion % 6283185307;

        // Divides by 1 million and rounds to the nearest integer.
        if angle_times_a_billion % 1000000000 > 499999999 {
            return_angle = (angle_times_a_billion / 1000000000 + 1) as i64;
        } else {
            return_angle = (angle_times_a_billion / 1000000000) as i64;
        }
        // Handles the case in which the angle is less than 2 pi thousandth radians.
    } else if return_angle < 0 {
        // Handles cases in which the angle is less than 2 pi radians.
        if return_angle < -6282 {
            // Multiplied by 1000000 to allow more precision.
            let mut angle_times_a_billion = i64::from(return_angle * 1000000000);

            // Normalizes angle
            angle_times_a_billion = angle_times_a_billion % -6283185307;

            // Divides by 1 million and rounds to the nearest integer.
            if angle_times_a_billion % 1000000000 < -499999999 {
                return_angle = (angle_times_a_billion / 1000000000 - 1) as i64;
            } else {
                return_angle = (angle_times_a_billion / 1000000000) as i64;
            }
        }

        // Converts negative angles into positive angles.
        return_angle = 6283 + return_angle;

        // Handles the case in which the angle is "exactly" 2 pi radians.
        if return_angle == 6283 {
            return_angle = 0;
        }
    }

    // Returns angle in thousanth angles.
    return return_angle;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denominator_to_1000() {
        test_equal_fraction((8.0, 1000.0), (8, 1000));
        test_equal_fraction((17.0, 1000.0), (17, 1000));
        test_equal_fraction((3.0, 7.0), (3, 7));
        test_equal_fraction((i32::MAX as f64, 1.0), (i32::MAX, 1));
        test_equal_fraction((i32::MIN as f64, 1.0), (i32::MIN, 1));
        test_equal_fraction((-4.0, 1000.0), (-4, 1000));
        test_equal_fraction((-19.0, 1000.0), (-19, 1000));
        test_equal_fraction((-3.0, 11.0), (-3, 11));
        test_equal_fraction((17.0, -11.0), (17, -11));

        for a in -10000..10001 {
            for b in -10000..0 {
                test_equal_fraction((a as f64, b as f64), (a, b));
            }
        }

        for a in -10000..10001 {
            for b in 1..10001 {
                test_equal_fraction((a as f64, b as f64), (a, b));
            }
        }
    }

    fn test_equal_fraction(float_fraction: (f64, f64), integer_fraction: (i32, i32)) {
        let test: bool;

        if
            (((float_fraction.0 / float_fraction.1) * 1000.0).round() as i64) ==
                denominator_to_1000(integer_fraction) ||
            (
                (
                    (float_fraction.0 / float_fraction.1) * 1000.0 -
                    (denominator_to_1000(integer_fraction) as f64)
                ).abs() - 0.5
            ).abs() < 0.00001
        {
            test = true;
        } else {
            test = false;
        }
        assert_eq!(test, true);
    }

    #[test]
    fn test_normalize_angle() {


        test_equal_angle(normalize_angle(-12568), angle_normalizer(-12568.0)); //fails
        
        /* for a in -7000..7000 {
            for b in -7000..7001 as i64 {
                test_equal_angle(
                    normalize_angle(a * 1000000),
                    angle_normalizer((b * 1000000) as f64)
                );
            }
        } */
    }

    fn angle_normalizer(thousandth_angle: f64) -> f64 {
        let mut angle = thousandth_angle / 1000.0;
        angle = angle.sin().asin();
        if angle < 0.0 {
            angle = angle + std::f64::consts::PI * 2.0;
        }

        return angle * 1000.0;
    }

    fn test_equal_angle(thousandth_integer_angle: i64, thousandth_float_angle: f64) {
        let test: bool;

        if
            (thousandth_float_angle.round() as i64) == thousandth_integer_angle ||
            ((thousandth_float_angle - (thousandth_integer_angle as f64)).abs() - 0.5).abs() <
                0.00001
        {
            test = true;
        } else {
            test = false;
            println!(" {} {} ", thousandth_integer_angle, thousandth_float_angle);
            println!("");
        }
        assert_eq!(test, true);
    }
}
