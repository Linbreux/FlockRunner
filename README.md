![FlockRunner logo](./logo.png)

⚠️ WARNING: FlockRunner is currently in active development and not yet ready for production use. Expect frequent changes and potential instability. ⚠️

FlockRunner is a powerful and highly customizable command executor designed to streamline your project workflows. It operates based on a simple YAML configuration file, giving you complete control over how your commands are organized and executed.

# Development

```
nix develop
cargo build
```

# The goal

I wanted a straightforward method for managing and running tools tailored to individual projects. Despite the probable existence of numerous solutions, I decided to develop my own as a practical way to learn Rust.

A project file should look like the example.

```yaml
project: "FlokeRunner"

variables:
  d: "date"
  greeting: "Hello from FlokeRunner!"

commands:
  # fr cmd hello -v
  # running hello command
  # hello brother
  hello:
    cmd: "echo hello brother"
    alias: "h"
    help: "a nice welcome message :)"
    # type: "default"

  # fr cmd time -v
  # running time command
  # <<current time>>
  time:
    cmd: "date"

  greet_date:
    cmd: "echo {{greeting}} Today's date is: \"$( {{d}} )\"" # {{d}} will be substituted, and then executed by shell.


sequence:
  # fr seq tell-time
  tell-time:
    - hello
    - time
```
