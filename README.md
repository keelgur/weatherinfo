# weatherinfo
A weather CLI for Windows/Linux/macOS, which is responsible for showing weather to a user.

# Preface
The following code was written to complete `Elastio Rust Test Task`. Unfortunately, the requirements weren't fully satisfied, mainly because of lack of free/open-source(even freemium) weather API that has ability to show weather by date - both past days(months,years) and future.
So, there's only one API provider, but command for configuring provider is still implemented.

# Instruction

## The application holds only two commands, which can be executed by handling following parameters in CLI after compiled executable argument:
```
weatherinfo get <address> [date=now]
weatherinfo configure <provider>
```
`[date]` is an optional parameter. If none is handled(None item), then current local date will be taken.
`<provider>` handled to the application is saved to file cfg.txt.
`<address>` must be entered in YYYY-mm-dd format(e.g. 2023-01-08).

  # Examples
  ```
  weatherinfo get London 2010-01-01
  API provider is set to: vc
  London, Temp: 0.3 °C, Precipitation probability: 0%, Humidity: 81.1%, Wind speed: 16.7km/h, Partly cloudy throughout the day.
  ```
  ```
  weatherinfo get London
  API provider is set to: vc
  London, Temp: 7.8 °C, Precipitation probability: 100%, Humidity: 83.5%, Wind speed: 25.6km/h, Partly cloudy throughout the day with rain.
  ```
  ```
  weatherinfo configure accuweather
  API provider is set to: vc
  Changed provider to accuweather
  ```
