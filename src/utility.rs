// Normalizes angles and converts to usize.

pub fn normalize_angle(angle: i32) -> usize {
    let mut return_angle = angle;

    // Handles the case in which the angle is greater than 2 * pi radians.
    if return_angle > 0 && return_angle > 6282 {
        // The following code deals with the fact that 2 * pi is not exactly 6283.
        let mut angle_times_a_million = i64::from(return_angle * 1000000);

        angle_times_a_million = angle_times_a_million % 6283185;

        if return_angle % 1000000 > 499999 {
            return_angle = (angle_times_a_million / 1000000) as i32;
        } else {
            return_angle = (angle_times_a_million / 1000000 + 1) as i32;
        }
        // Handles the case in which the angle is less than 2 pi radians.
    } else if return_angle < 0 {
        //Handles cases in which the angle is less than 2 pi radians.
        if return_angle < -6283 {
            // The following code deals with the fact that 2 * pi is not exactly 6283.
            let mut angle_times_a_million = i64::from(return_angle * 1000000);

            angle_times_a_million = angle_times_a_million % 6283185;

            if return_angle % 1000000 > 499999 {
                return_angle = (angle_times_a_million / 1000000) as i32;
            } else {
                return_angle = (angle_times_a_million / 1000000 + 1) as i32;
            }
        }

        // Converts negative angles into positive angles.
        return_angle = 6283 + return_angle;

        // Handles the case in which the angle is "exactly" 2 pi radians.
        if return_angle == 6283 {
            return_angle = 0;
        }
    }
    return return_angle as usize;
}
