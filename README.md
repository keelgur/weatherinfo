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
`date` is an optional parameter. If none is handled(None item), then current local date will be taken.
