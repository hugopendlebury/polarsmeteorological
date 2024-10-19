use crate::meteoconversions::*;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

#[polars_expr(output_type=Float64)]
fn celsius_dew_point(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_dew_point(inputs)
}

#[polars_expr(output_type=Float64)]
fn celsius_to_fahrenheit(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_to_fahrenheit(inputs)
}

#[polars_expr(output_type=Float64)]
fn celsius_to_kelvin(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_to_kelvin(inputs)
}

#[polars_expr(output_type=Float64)]
fn kelvin_to_celsius(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_kelvin_to_celsius(inputs)
}

#[polars_expr(output_type=Float64)]
fn kelvin_to_fahrenheit(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_kelvin_to_fahrenheit(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_to_kelvin(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_to_kelvin(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_celsius_humidex(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_celsius_humidex(inputs)
}


#[polars_expr(output_type=Float64)]
fn fahrenheit_to_celsius(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_to_celsius(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_celsius_dew_point(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_celsius_dew_point(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_fahrenheit_dew_point(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_fahrenheit_dew_point(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_dew_point(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_dew_point(inputs)
}

#[polars_expr(output_type=Float64)]
fn celsius_absolute_humidity(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_absolute_humidity(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_absolute_humidity(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_absolute_humidity(inputs)
}


#[polars_expr(output_type=Float64)]
fn celsius_heat_index(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_heat_index(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_heat_index(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_heat_index(inputs)
}

#[polars_expr(output_type=Float64)]
fn celsius_humidex(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_humidex(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_fahrenheit_humidex(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_fahrenheit_humidex(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_humidex(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_humidex(inputs)
}

#[polars_expr(output_type=Float64)]
fn celsius_mixing_ratio(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_celsius_mixing_ratio(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_celsius_mixing_ratio(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_celsius_mixing_ratio(inputs)
}

#[polars_expr(output_type=Float64)]
fn common_fahrenheit_mixing_ratio(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_common_fahrenheit_mixing_ratio(inputs)
}

#[polars_expr(output_type=Float64)]
fn fahrenheit_mixing_ratio(inputs: &[Series]) -> Result<Series, PolarsError> {

    impl_fahrenheit_mixing_ratio(inputs)
}