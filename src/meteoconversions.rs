use meteo_tools::*;
use polars::series::Series;
use polars::prelude::*;
use itertools::izip;
use ordered_float::Pow;

pub(crate) fn impl_common_celsius_dew_point(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_celsius_dew_point)

}

pub(crate) fn impl_common_fahrenheit_dew_point(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_fahrenheit_dew_point)

}

pub(crate) fn impl_fahrenheit_dew_point(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_three_arg_series_generic(inputs, fahrenheit_dew_point)

}

pub(crate) fn impl_celsius_heat_index(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, celsius_heat_index)

}

pub(crate) fn impl_fahrenheit_heat_index(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, fahrenheit_heat_index)

}

pub(crate) fn impl_common_fahrenheit_humidex(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_fahrenheit_humidex)

}

pub(crate) fn impl_fahrenheit_humidex(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_three_arg_series_generic(inputs, fahrenheit_humidex)

}

pub(crate) fn impl_common_fahrenheit_mixing_ratio(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_fahrenheit_mixing_ratio)

}

pub(crate) fn impl_fahrenheit_mixing_ratio(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_three_arg_series_generic(inputs, fahrenheit_mixing_ratio)

}

pub(crate) fn impl_celsius_mixing_ratio(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_three_arg_series_generic(inputs, celsius_mixing_ratio)

}

pub(crate) fn impl_common_celsius_mixing_ratio(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_celsius_mixing_ratio)

}

pub(crate) fn impl_celsius_humidex(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_three_arg_series_generic(inputs, celsius_humidex)

}

pub(crate) fn impl_common_celsius_humidex(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, common_celsius_humidex)

}

pub(crate) fn impl_celsius_to_fahrenheit(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, celsius_to_fahrenheit)

}

pub(crate) fn impl_celsius_to_kelvin(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, celsius_to_kelvin)

}

pub(crate) fn impl_kelvin_to_celsius(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, kelvin_to_celsius)

}

pub(crate) fn impl_kelvin_to_fahrenheit(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, kelvin_to_fahrenheit)

}

pub(crate) fn impl_fahrenheit_to_kelvin(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, fahrenheit_to_kelvin)

}

pub(crate) fn impl_fahrenheit_to_celsius(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, fahrenheit_to_celsius)

}

pub(crate) fn impl_celsius_absolute_humidity(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, celsius_absolute_humidity)

}

pub(crate) fn impl_hpa_to_inhg(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, hpa_to_inhg)

}

pub(crate) fn impl_hpa_to_mmhg(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, hpa_to_mmhg)

}

pub(crate) fn impl_inhg_to_hpa(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, inhg_to_hpa)

}

pub(crate) fn impl_kmph_to_mph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, kmph_to_mph)

}

pub(crate) fn impl_knots_to_kmph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, knots_to_kmph)

}

pub(crate) fn impl_knots_to_mps(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, knots_to_mps)

}

pub(crate) fn impl_mph_to_knots(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mph_to_knots)

}

pub(crate) fn impl_mph_to_mps(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mph_to_mps)

}

pub(crate) fn impl_mps_to_kmph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mps_to_kmph)

}

pub(crate) fn impl_mps_to_mph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mps_to_mph)

}

pub(crate) fn impl_mps_to_knots(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mps_to_knots)

}

pub(crate) fn impl_mph_to_kmph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mph_to_kmph)

}

pub(crate) fn impl_knots_to_mph(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, knots_to_mph)

}

pub(crate) fn impl_kmph_to_mps(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, kmph_to_mps)

}

pub(crate) fn impl_kmph_to_knots(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, kmph_to_knots)

}

pub(crate) fn impl_mmhg_to_hpa(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_single_arg_series_generic(inputs, mmhg_to_hpa)

}

pub(crate) fn impl_fahrenheit_absolute_humidity(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 

    impl_two_arg_series_generic(inputs, fahrenheit_absolute_humidity)

}


pub(crate) fn impl_single_arg_series_generic(
    inputs: &[Series],
    func: fn(&f64) -> f64
) -> Result<Series, PolarsError> { 

    let temperatures = &inputs[0];
    let relative_humidities = &inputs[1];
    let atmospheric_pressures = &inputs[2];

    let points = izip!(
        temperatures.f64()?.into_iter(),
        relative_humidities.f64()?.into_iter(),
        atmospheric_pressures.f64()?.into_iter()
    );

    let values  = points.map(|x| {
        x.0.map(|t| func(&t))
    });

    Ok(Series::new("ts", values.collect::<Vec<_>>()))


}

fn impl_wind_speed(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_two_arg_series_generic(inputs, |u: &f64, v| {
        let a : f64 = u.pow(2) + v.pow(2);
        a.sqrt()
    })
}

pub(crate) fn impl_two_arg_series_generic(
    inputs: &[Series],
    func: fn(&f64, &f64) -> f64
) -> Result<Series, PolarsError> { 

    let temperatures = &inputs[0];
    let relative_humidities = &inputs[1];
    let atmospheric_pressures = &inputs[2];

    let points = izip!(
        temperatures.f64()?.into_iter(),
        relative_humidities.f64()?.into_iter(),
        atmospheric_pressures.f64()?.into_iter()
    );

    let values  = points.map(|x| {

        if let Some(t) = x.0 {
            x.1.map(|r| func(&t, &r))
        } else {
            None
        }
    });

    Ok(Series::new("ts", values.collect::<Vec<_>>()))


}

pub(crate) fn impl_celsius_dew_point(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 
    impl_three_arg_series_generic(inputs, celsius_dew_point)
}


pub(crate) fn impl_three_arg_series_generic(
    inputs: &[Series],
    func: fn(&f64, &f64, &f64) -> f64
) -> Result<Series, PolarsError> { 

    let temperatures = &inputs[0];
    let relative_humidities = &inputs[1];
    let atmospheric_pressures = &inputs[2];

    let points = izip!(
        temperatures.f64()?.into_iter(),
        relative_humidities.f64()?.into_iter(),
        atmospheric_pressures.f64()?.into_iter()
    );

    let values  = points.map(|x| {

        if let Some(t) = x.0 {
            if let Some(r) = x.1 {
                x.2.map(|p| func(&t, &r, &p))
            } else {
                None
            }
        } else {
            None
        }
    });

    Ok(Series::new("ts", values.collect::<Vec<_>>()))


}
