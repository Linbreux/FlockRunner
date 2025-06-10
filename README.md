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

![example with the config bellow](./vhs/example.gif)
```yaml
project: "FlockRunner"

variables:
  greeting: "Hello from FlockRunner!"
  user: "Flock user"

commands:
  hello:
    cmd: "echo hello {{user}}"
    alias: "h"
    # type: "default"

  time:
    cmd: date

  clean:
    help: "cleanup"
    cmd:
      - rm -r flock && echo removed!

  create:
    help: "create a text file inside a flock folder"
    cmd:
      - echo "creating folder"
      - mkdir flock
      - echo "new textfile" > flock/test.txt && echo created textfile
      - echo "done running, bye {{user}}"

  greet_date:
    cmd: echo "{{greeting}}" && date


sequence:
  # fr seq tell-time
  tell-time:
    - hello
    - time
```
