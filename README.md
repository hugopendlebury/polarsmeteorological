# Polars-Meteorological

Meteorological functions for [Polars](https://www.pola.rs/).

- ✅ blazingly fast, written in Rust!
- ✅ seamless Polars integration!
- ✅ Conversions
    - ✅ Celcius to Fahrenheit
    - ✅ Celcius to Kelvin
    - ✅ Fahrenheit to Celcius
    - ✅ Fahrenheit to Kelix
- ✅ Calculcations
    - ✅ Heat index
    - ✅ Humidty


Installation
------------

First, you need to [install Polars](https://pola-rs.github.io/polars/user-guide/installation/).

Then, you'll need to install `polarsmeteorological`:
```console
pip install polarsmeteorological
```

Usage
-------------
The module creates a custom namespace which is attached to a polars expression

What does this mean. If you are using polars regularly you will be aware of the .str and .arr 
(now renamed .list) namespaces which have special functions related to string and arrays / lists

This module creates a custom namespace 

