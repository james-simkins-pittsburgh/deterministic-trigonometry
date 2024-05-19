// Normalizes angles and converts to usize.

pub fn normalize_angle(angle: i64) -> usize {
    let mut return_angle = angle;

    // Handles the case in which the angle is greater than 2 * pi radians.
    if return_angle > 0 && return_angle > 6282 {
        // Multiplied by 1000000 to allow more precision.
        let mut angle_times_a_million = i128::from(return_angle * 1000000);

        // Normalizes angle
        angle_times_a_million = angle_times_a_million % 6283185;

        // Divides by 1 million and rounds to the nearest integer.
        if return_angle % 1000000 > 499999 {
            return_angle = (angle_times_a_million / 1000000 + 1) as i64;
        } else {
            return_angle = (angle_times_a_million / 1000000) as i64;
        }
        // Handles the case in which the angle is less than 2 pi radians.
    } else if return_angle < 0 {
        //Handles cases in which the angle is less than 2 pi radians.
        if return_angle < -6282 {
            // Multiplied by 1000000 to allow more precision.
            let mut angle_times_a_million = i64::from(return_angle * 1000000);

            // Normalizes angle
            angle_times_a_million = angle_times_a_million % -6283185;

            // Divides by 1 million and rounds to the nearest integer.
            if return_angle % 1000000 > 499999 {
                return_angle = (angle_times_a_million / 1000000 + 1) as i64;
            } else {
                return_angle = (angle_times_a_million / 1000000) as i64;
            }
        }

        // Converts negative angles into positive angles.
        return_angle = 6283 + return_angle;

        // Handles the case in which the angle is "exactly" 2 pi radians.
        if return_angle == 6283 {
            return_angle = 0;
        }
    }

    // Returns angle as usize so it can be used as an index for the arrays.
    return return_angle as usize;
}

// Outputs the nearest integer fraction out of 1000 to the argument fraction.
pub fn denominator_to_1000(argument_fraction: (i32, i32)) -> i64 {
    // No need to do anything if input is already out of 1000.
    if  argument_fraction.1 == 1000 {
        return i64::from(argument_fraction.0);
    } else {
        if argument_fraction.1 % 2 == 0 {
            if (argument_fraction.0 * 1000) % argument_fraction.1 >= argument_fraction.1 / 2 {
                return i64::from((argument_fraction.0 * 1000) / argument_fraction.1 + 1);
            } else {
                return i64::from((argument_fraction.0 * 1000) / argument_fraction.1);
            }
        } else {
            if (argument_fraction.0 * 1000) % argument_fraction.1 > argument_fraction.1 / 2 {
                return i64::from((argument_fraction.0 * 1000) / argument_fraction.1 + 1);
            } else {
                return i64::from((argument_fraction.0 * 1000) / argument_fraction.1);
            }
        }
    }
}
