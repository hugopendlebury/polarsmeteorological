from __future__ import annotations

import polarsmeteorological.namespace  # noqa: F401
from polarsmeteorological.functions import (
    celsius_dew_point,
    common_celsius_humidex,
    common_celsius_dew_point,
    common_fahrenheit_dew_point,
    fahrenheit_dew_point,
    celsius_mixing_ratio,
    fahrenheit_mixing_ratio,
    common_celsius_mixing_ratio,
    common_fahrenheit_mixing_ratio,
    celsius_absolute_humidity,
    fahrenheit_absolute_humidity,
    celsius_heat_index,
    fahrenheit_heat_index,
    celsius_to_fahrenheit,
    fahrenheit_to_celsius,
    celsius_humidex,
    fahrenheit_humidex,
    fahrenheit_to_kelvin,
    celsius_to_kelvin,
    kelvin_to_celsius,
    kelvin_to_fahrenheit,
    hpa_to_inhg,
    inhg_to_hpa,
    mmhg_to_hpa,
    kmph_to_mph,
    kmph_to_mps,
    knots_to_kmph,
    knots_to_mph,
    mph_to_kmph,
    mph_to_knots,
    mph_to_mps,
    mps_to_kmph,
    mps_to_knots,
    mps_to_mph,
    wind_speed,
    wind_chill_celsius
)

#from ._internal import __version__

__all__ = [
    "celsius_dew_point",
    "common_celsius_humidex",
    "common_celsius_dew_point",
    "common_fahrenheit_dew_point",
    "fahrenheit_dew_point",
    "celsius_mixing_ratio",
    "fahrenheit_mixing_ratio",
    "common_celsius_mixing_ratio",
    "common_fahrenheit_mixing_ratio",
    "celsius_absolute_humidity",
    "fahrenheit_absolute_humidity",
    "celsius_heat_index",
    "fahrenheit_heat_index",
    "celsius_to_fahrenheit",
    "fahrenheit_to_celsius",
    "celsius_humidex",
    "fahrenheit_humidex",
    "fahrenheit_to_kelvin",
    "celsius_to_kelvin",
    "kelvin_to_celsius",
    "kelvin_to_fahrenheit",
    "hpa_to_inhg",
    "inhg_to_hpa",
    "mmhg_to_hpa",
    "kmph_to_mph",
    "kmph_to_mps",
    "knots_to_kmph",
    "knots_to_mph",
    "mph_to_kmph",
    "mph_to_knots",
    "mph_to_mps",
    "mps_to_kmph",
    "mps_to_knots",
    "mps_to_mph",
    "wind_speed",
    "wind_chill_celsius",
    "__version__"
]