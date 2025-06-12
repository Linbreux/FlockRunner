![FlockRunner logo](./logo.png)

⚠️ WARNING: FlockRunner is currently in active development and not yet ready for production use. Expect frequent changes and potential instability. ⚠️

FlockRunner is a powerful and highly customizable command executor designed to streamline your project workflows. It operates based on a simple YAML configuration file, giving you complete control over how your commands are organized and executed.

# Installation

Currently only nix is supported:

```
nix profile install github:Linbreux/FlockRunner
```
or (running it directly)
```
nix run github:Linbreux/FlockRunner
```

# Development

```
nix develop
cargo build
```

# The goal

I wanted a straightforward method for managing and running tools tailored to individual projects. Despite the probable existence of numerous solutions, I decided to develop my own as a practical way to learn Rust.

You can place your flockrunner.yaml file right in your project's main folder. No matter where you're running FlockRunner from within your project's hierarchy, it'll automatically look for that configuration file by searching parent directories. The first one it finds on its way up is the one it'll use, ensuring you're always using the correct project configuration.

A project file should look like the example.

![example with the config bellow](./vhs/example.gif)
```yaml
project: "FlockRunner"

variables:
  greeting: "Hello from FlockRunner!"
  user: "Flock user"

shells:
  # A shell definition to run commands inside a Docker container
  docker-shell: docker run {{os}} sh -c 
  # An alternative shell using zsh
  zsh: zsh -c 

commands:
  hello:
    cmd: echo hello {{user}}
    alias: "h"
    help: "Greets the current user."

  time:
    cmd: date
    help: "Shows the current system date and time."

  create:
    help: "Creates a 'flock' directory and a 'test.txt' file within it."
    cmd:
      - echo "Creating folder..."
      - mkdir flock
      - "echo new textfile > flock/test.txt && echo Created textfile: flock/test.txt"
      - echo "Done creating files, bye {{user}}!"

  clean:
    help: "Removes the 'flock' directory and its contents."
    keep_going: true # Continues execution even if `rm` fails (e.g., directory doesn't exist)
    cmd:
      - rm -rf flock && echo "Removed 'flock' directory!" || echo "'flock' directory not found or could not be removed."

  greet_date:
    help: "Displays a greeting, the current date, and then exits with an error."
    cmd: echo {{greeting}} && date && exit 1 # cmd will file because of exit code.

  run_on_os:
    help: "Runs a command inside a specified Docker container (defaulting to fedora)."
    # Uses the 'docker-shell' defined above (shells)
    shell: docker-shell
    variables:
      os: fedora # Overrides the project's 'os' variable for this command only
    cmd:
      - echo Running 'cat /etc/os-release' inside {{os}} container...
      - cat /etc/os-release
      - echo == Command executed within the {{os}} container ==

sequence:
```
