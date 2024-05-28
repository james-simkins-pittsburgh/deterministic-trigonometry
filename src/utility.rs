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

pub fn normalize_angle(thousandth_angle: i64) -> i64 {
    let mut return_angle = thousandth_angle;

    // Handles the case in which the angle is greater than or equal to 2 pi radians.
    if return_angle > 6282 {
        // Multiplied by 1000000 to allow more precision.
        let mut angle_times_a_billion = i128::from(return_angle) * 1000000000;

        // Normalizes angle
        angle_times_a_billion = angle_times_a_billion % 6283185307180;

        // Divides by 1 million and rounds to the nearest integer.
        if angle_times_a_billion % 1000000000 > 499999999 {
            return_angle = (angle_times_a_billion / 1000000000 + 1) as i64;
        } else {
            return_angle = (angle_times_a_billion / 1000000000) as i64;
        }

        // Handles negative angles.
    } else if return_angle < 0 {
        // Multiplied by 1 billion to allow more precision.
        let mut angle_times_a_billion = i128::from(return_angle) * 1000000000;

        // Normalizes angle
        angle_times_a_billion = angle_times_a_billion % 6283185307180;

        // Converts negative angle into positive angle.
        angle_times_a_billion = angle_times_a_billion + 6283185307180;

        // Divides by 1 billion and rounds to the nearest integer.
        if angle_times_a_billion % 1000000000 > 499999999 {
            return_angle = (angle_times_a_billion / 1000000000 + 1) as i64;
        } else {
            return_angle = (angle_times_a_billion / 1000000000) as i64;
        }
    }

    // Handles the case in which the angle is "exactly" 2 pi radians.
    if return_angle == 6283 {
        return_angle = 0;
    }

    // Returns angle in thousandth angle.
    return return_angle;
}

#[cfg(test)]
mod tests {
    use super::*;

    // This tests to make sure the denominator to 1000 works in a range of cases.
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
            // This excludes various exceptions due to rounding errors.
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
        test_equal_angle(normalize_angle(7000), angle_normalizer(7000.0), 7000);
        test_equal_angle(normalize_angle(-12568), angle_normalizer(-12568.0), -12568);

        for a in -6284..6284 {
            test_equal_angle(normalize_angle (a * 1000000), angle_normalizer((a *1000000) as f64), a);
        }
    }

    fn angle_normalizer(thousandth_angle: f64) -> f64 {
        let mut angle = thousandth_angle / 1000.0;
        angle = angle % (2.0 * std::f64::consts::PI);
        if angle < 0.0 {

            angle = angle + 2.0 * std::f64::consts::PI;
        }

        return angle * 1000.0;
    }

    fn test_equal_angle(thousandth_integer_angle: i64, thousandth_float_angle: f64, number: i64) {
        let test: bool;

        if
            (thousandth_float_angle.round() as i64) == thousandth_integer_angle ||
            // This excludes various exceptions due to rounding errors.
            ((thousandth_float_angle - (thousandth_integer_angle as f64)).abs() - 0.5).abs() <
                0.001 || (thousandth_float_angle.round() as i64 == 6283 && thousandth_integer_angle == 0)
        {
            test = true;
        } else {
            test = false;
            println!(" {} {} {} ", number, thousandth_integer_angle, thousandth_float_angle);
            println!("");
        }
        assert_eq!(test, true);
    }
}
