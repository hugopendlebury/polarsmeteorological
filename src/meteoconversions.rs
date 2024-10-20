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

pub(crate) fn impl_wind_speed(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_two_arg_series_generic(inputs, |u: &f64, v| {
        let a : f64 = u.pow(2) + v.pow(2);
        a.sqrt()
    })
}

pub(crate) fn impl_meters_to_feet(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_single_arg_series_generic(inputs, |m| {
        m * 3.28084
    })

}

pub(crate) fn impl_wind_chill_celsius_kph(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_two_arg_series_generic(inputs, |t, w| {
        13.12 + 0.6215*t - 11.37 * w.pow(0.16) + 0.3965 * t*w.pow(0.16)
    })

}

pub(crate) fn impl_celsius_dew_point(
    inputs: &[Series],
) -> Result<Series, PolarsError> { 
    impl_three_arg_series_generic(inputs, celsius_dew_point)
}

pub(crate) fn impl_single_arg_series_generic(
    inputs: &[Series],
    func: fn(&f64) -> f64
) -> Result<Series, PolarsError> { 

    let values = &inputs[0];
    let values_iter = values.f64()?.into_iter();

    let results = values_iter.map(|x| {
        x.map(|t| func(&t))
    });

    Ok(Series::new("ts", results.collect::<Vec<_>>()))


}

pub(crate) fn impl_two_arg_series_generic(
    inputs: &[Series],
    func: fn(&f64, &f64) -> f64
) -> Result<Series, PolarsError> { 

    let temperatures = &inputs[0];
    let relative_humidities = &inputs[1];

    let points = izip!(
        temperatures.f64()?.into_iter(),
        relative_humidities.f64()?.into_iter()
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

#[cfg(test)]
mod test {
    use polars::series::Series;
    use polars::prelude::*;
    use crate::meteoconversions::*;

    #[test]
    fn test_celsius_dew_point() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[14.9477]);
    
        let results = impl_common_celsius_dew_point(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_celsius_dew_point() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[14.9477]);
    
        let results = impl_common_celsius_dew_point(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_fahrenheit_dew_point() {


        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[58.9059]);
    
        let results = impl_common_fahrenheit_dew_point(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }


    #[test]
    fn test_fahrenheit_dew_point() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[58.9059]);
    
        let results = impl_fahrenheit_dew_point(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_heat_index() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[24.4298]);
    
        let results = impl_celsius_heat_index(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_fahrenheit_heat_index() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[75.9737]);
    
        let results = impl_fahrenheit_heat_index(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_fahrenheit_humidex() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[99.6699]);
    
        let results = impl_common_fahrenheit_humidex(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_fahrenheit_humidex() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[99.6699]);
    
        let results = impl_fahrenheit_humidex(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_fahrenheit_mixing_ratio() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[0.0826]);
    
        let results = impl_common_fahrenheit_mixing_ratio(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_fahrenheit_mixing_ratio() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[0.0826]);
    
        let results = impl_fahrenheit_mixing_ratio(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_mixing_ratio() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[0.0826]);
    
        let results = impl_celsius_mixing_ratio(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_celsius_mixing_ratio() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[0.0826]);
    
        let results = impl_common_celsius_mixing_ratio(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_humidex() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let atmospheric_pressure = Series::new("atmospheric_pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[25.2487]);
    
        let results = impl_celsius_humidex(&[temperature, relative_humidity, atmospheric_pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_common_celsius_humidex() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humitidy".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[25.2487]);
    
        let results = impl_common_celsius_humidex(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_to_fahrenheit() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let expected = Series::new("ts".into(), &[72.5]);
    
        let results = impl_celsius_to_fahrenheit(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_to_kelvin() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let expected = Series::new("ts".into(), &[295.65]);
    
        let results = impl_celsius_to_kelvin(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_kelvin_to_celsius() {

        let temperature = Series::new("temperature".into(), &[295.65]);
        let expected = Series::new("ts".into(), &[22.5]);
    
        let results = impl_kelvin_to_celsius(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_kelvin_to_fahrenheit() {

        let temperature = Series::new("temperature".into(), &[295.65]);
        let expected = Series::new("ts".into(), &[72.5]);
    
        let results = impl_kelvin_to_fahrenheit(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_fahrenheit_to_kelvin() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let expected = Series::new("ts".into(), &[295.65]);
    
        let results = impl_fahrenheit_to_kelvin(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_fahrenheit_to_celsius() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let expected = Series::new("ts".into(), &[22.5]);
    
        let results = impl_fahrenheit_to_celsius(&[temperature]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_celsius_absolute_humidity() {

        let temperature = Series::new("temperature".into(), &[22.5]);
        let relative_humidity = Series::new("relative_humidity".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[0.0871]);
    
        let results = impl_celsius_absolute_humidity(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));

    }

    #[test]
    fn test_hpa_to_inhg() {

        let pressure = Series::new("pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[29.9212]);
    
        let results = impl_hpa_to_inhg(&[pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_hpa_to_mmhg() {

        let pressure = Series::new("pressure".into(), &[1013.25]);
        let expected = Series::new("ts".into(), &[760.0003]);
    
        let results = impl_hpa_to_mmhg(&[pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }
    
    #[test]
    fn test_inhg_to_hpa() {

        let pressure = Series::new("pressure".into(), &[29.9212]);
        let expected = Series::new("ts".into(), &[1013.2485]);
    
        let results = impl_inhg_to_hpa(&[pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }
    
    #[test]
    fn test_kmph_to_mph() {

        let speed = Series::new("speed".into(), &[36.0]);
        let expected = Series::new("ts".into(), &[22.3694]);
    
        let results = impl_kmph_to_mph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_knots_to_kmph() {

        let speed = Series::new("speed".into(), &[5.3996]);
        let expected = Series::new("ts".into(), &[10.0001]);
    
        let results = impl_knots_to_kmph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_knots_to_mps() {

        let speed = Series::new("speed".into(), &[19.4384]);
        let expected = Series::new("ts".into(), &[10.0]);
    
        let results = impl_knots_to_mps(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mph_to_knots() {

        let speed = Series::new("speed".into(), &[22.3694]);
        let expected = Series::new("ts".into(), &[19.4385]);
    
        let results = impl_mph_to_knots(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mph_to_mps() {

        let speed = Series::new("speed".into(), &[10.0]);
        let expected = Series::new("ts".into(), &[4.4704]);
    
        let results = impl_mph_to_mps(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mps_to_kmph() {

        let speed = Series::new("speed".into(), &[10.0]);
        let expected = Series::new("ts".into(), &[36.0]);
    
        let results = impl_mps_to_kmph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }
    
    #[test]
    fn test_mps_to_mph() {

        let speed = Series::new("speed".into(), &[10.0]);
        let expected = Series::new("ts".into(), &[22.3694]);
    
        let results = impl_mps_to_mph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mps_to_knots() {

        let speed = Series::new("speed".into(), &[10.0]);
        let expected = Series::new("ts".into(), &[19.4384]);
    
        let results = impl_mps_to_knots(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mph_to_kmph() {

        let speed = Series::new("speed".into(), &[22.3694]);
        let expected = Series::new("ts".into(), &[36.0001]);
    
        let results = impl_mph_to_kmph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_knots_to_mph() {

        let speed = Series::new("speed".into(), &[19.4385]);
        let expected = Series::new("ts".into(), &[22.3694]);
    
        let results = impl_knots_to_mph(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }
    
    #[test]
    fn test_kmph_to_mps() {

        let speed = Series::new("speed".into(), &[36.0]);
        let expected = Series::new("ts".into(), &[10.0]);
    
        let results = impl_kmph_to_mps(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_kmph_to_knots() {

        let speed = Series::new("speed".into(), &[10.0]);
        let expected = Series::new("ts".into(), &[5.3996]);
    
        let results = impl_kmph_to_knots(&[speed]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_mmhg_to_hpa() {

        let pressure = Series::new("pressure".into(), &[760.0003]);
        let expected = Series::new("ts".into(), &[1013.25]);
    
        let results = impl_mmhg_to_hpa(&[pressure]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }
    
    #[test]
    fn test_fahrenheit_absolute_humidity() {

        let temperature = Series::new("temperature".into(), &[72.5]);
        let relative_humidity = Series::new("relative_humidity".into(), &[62.4]);
        let expected = Series::new("ts".into(), &[0.0871]);
    
        let results = impl_fahrenheit_absolute_humidity(&[temperature, relative_humidity]);

        let unwrapped = results.unwrap();

        assert!(expected.series_equal(&unwrapped));
    }

    #[test]
    fn test_wind_chill_celsius_kph() {

        let temperature = Series::new("temperature".into(), &[-5.0]);
        let wind_speed = Series::new("temperature".into(), &[20.0]);
        let expected = Series::new("ts".into(), &[8]);
    
        let results = impl_wind_chill_celsius_kph(&[temperature, wind_speed]);

        let unwrapped = results.unwrap();

        println!("{}", unwrapped);

        assert!(expected.series_equal(&unwrapped));
    }



}