## FlockRunner Configuration File Structure (`flockrunner.yaml`)

This document outlines the structure of the **`flockrunner.yaml`** file. This file defines how FlockRunner behaves, allowing you to set project-specific variables, define custom commands, and group those commands into sequences.

---
### Top-Level Keys

Your `flockrunner.yaml` configuration file is organized under the following top-level keys:

* **`project`**: Defines the name of your project.
* **`variables`**: A collection of key-value pairs that can be used throughout your commands.
* **`commands`**: A mapping where each key represents a unique command name, and its value defines the command's behavior.
* **`sequence`**: A mapping where each key defines a sequence name, and its value is an ordered list of command names to execute.

At the moment all these keys are required to start a project.

---
### Detailed Structure

#### `project`
This key holds a single string value representing the name of your project. It's primarily used for identification and context.

* **Type**: String
* **Example**: `project: "FlockRunner"`

#### `variables`
This section allows you to define custom **variables** that can be referenced within your commands. Each variable is a simple key-value pair.

* **Type**: Map (key-value pairs)
* **Usage in commands**: Variables are interpolated using the `{{variable_name}}` syntax.
* **Example**:
    ```yaml
    variables:
      greeting: "Hello from FlockRunner!"
      user: "Flock user"
    ```

#### `commands`
This is where you define individual, executable actions. Each entry under `commands` is a unique **command name** that maps to a set of properties defining its execution.

* **Type**: Map (command name to command properties)

    Each **command name** (e.g., `hello`, `time`, `clean`) can have the following properties:

    * **`cmd`**: The actual command(s) to be executed.
        * **Type**: String or List of Strings
        * **Description**: Can be a single shell command string, or a list of commands that will be executed in order.
        * **Example**:
            ```yaml
            hello:
              cmd: "echo hello {{user}}"
            clean:
              cmd:
                - rm -r flock
                - echo removed!
            ```

    * **`alias`**: An optional shorter name for the command.
        * **Type**: String
        * **Description**: Allows you to invoke the command using a more concise alias.
        * **Example**: `alias: "h"`

    * **`help`**: An optional descriptive text for the command.
        * **Type**: String
        * **Description**: Provides a brief explanation of what the command does, typically displayed when listing commands.
        * **Example**: `help: "cleanup"`

    * **`keep_going`**: Controls error handling for multi-step commands.
        * **Type**: Boolean
        * **Description**: If `true`, FlockRunner will continue executing subsequent commands in a `cmd` list even if a previous command fails. If `false` (or omitted, which defaults to `false`), execution will stop on the first error within the `cmd` list. This only applies to the `cmd` property, not across different commands in a `sequence`.
        * **Example**: `keep_going: true`

#### `sequence`
This section allows you to define a series of commands to be run in a specific order. Each entry under `sequence` is a **sequence name** that maps to an ordered list of **command names**.

* **Type**: Map (sequence name to list of command names)
* **Description**: When a sequence is executed, FlockRunner will run each command listed under it, in the order they appear.
* **Example**:
    ```yaml
    sequence:
      tell-time:
        - hello
        - time
    ```

---
This structured `flockrunner.yaml` file empowers you to define complex workflows and easily manage your project's command-line operations with FlockRunner.
