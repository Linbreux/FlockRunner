# FlockRunner Project: Configuration Structure and Features with Examples

This document explains the core structure and key features of a **FlockRunner** configuration file, using concise examples to illustrate each concept. FlockRunner is designed to parse YAML files like this to define and execute commands, providing a flexible way to automate tasks.

## Configuration Structure

A FlockRunner configuration file is organized into several top-level sections:

* **`project`**:

    * Defines the overall name for your configuration.

    * **Example**:

        ```yaml
        project: "MyWebApp"

        ```

* **`variables`**:

    * Allows you to define reusable values that can be inserted into your commands. These enhance flexibility.

    * You reference variables using `{{variable_name}}`.

    * You can also override these variables directly from the command line using the `fr` tool.

    * **Example**:

        ```yaml
        variables:
          app_name: "AuthService"
          user_email: "dev@example.com"

        ```

    * **CLI Example**: To override `user_email` when running a command:

        ```bash
        fr cmd some_command -D user_email=new.user@example.com

        ```

* **`shells`**:

    * Defines custom execution environments for your commands, beyond the default system shell.

    * This is useful for running commands within specific interpreters (like `zsh`) or container environments (like Docker).

    * **Example**:

        ```yaml
        shells:
          docker-run: docker run --rm -it {{image}} sh -c
          bash-strict: bash -c -e

        ```

* **`commands`**:

    * This section defines the individual tasks or operations FlockRunner can execute.

    * Each command has a unique name and various properties to control its behavior.

    * You run these commands using the `fr cmd <command_name>` command.

    * **Example**:

        ```yaml
        commands:
          install-deps:
            cmd: npm install
            help: "Installs project dependencies."

        ```

    * **CLI Example**: To run the `install-deps` command:

        ```bash
        fr cmd install-deps

        ```

## Key Command Features

Within the `commands` section, each individual command definition can leverage several features:

* **`cmd`**:

    * **Purpose**: The actual command-line instruction(s) to be executed.

    * **Format**: Can be a single string for a single command, or a **list of strings** for multiple sequential commands.

    * **Example (Single Command)**:

        ```yaml
        my_command:
          cmd: echo "Hello, World!"
        ```

    * **CLI Example**: To run `my_command`:

        ```bash
        fr cmd my_command
        ```

    * **Example (List of Commands)**:

        ```yaml
        setup_project:
          cmd:
            - mkdir my_project
            - cd my_project
            - touch README.md
            - echo "Project initialized."
        ```

    * **CLI Example**: To run `setup_project`:

        ```bash
        fr cmd setup_project
        ```

* **`help`**:

    * **Purpose**: A descriptive string explaining what the command does.

    * **Example**:

        ```yaml
        deploy_app:
          cmd: ./deploy.sh
          help: "Deploys the application to production."
        ```

* **`alias`**:

    * **Purpose**: Defines a shorter, alternative name for the command, making it quicker to type.

    * **Example**:

        ```yaml
        status_check:
          cmd: git status
          alias: "gs"
          help: "Shows Git repository status."
        ```

    * **CLI Example**: To run `status_check` using its alias:

        ```bash
        fr cmd gs
        ```

* **`keep_going`**:

    * **Purpose**: A boolean flag (`true` or `false`). If `true`, FlockRunner will continue executing subsequent commands in a sequence (if a sequence feature were implemented) even if *this* command fails (exits with a non-zero status code). By default, a failing command would typically stop execution.

    * **Example**:

        ```yaml
        cleanup_old_logs:
          cmd: rm -rf /var/log/old_logs
          keep_going: true # Continue even if logs directory doesn't exist
          help: "Removes old log files, ignores errors if directory is missing."
        ```

    * **CLI Example**: To run `cleanup_old_logs`:

        ```bash
        fr cmd cleanup_old_logs
        ```

* **`shell`**:

    * **Purpose**: Specifies which shell definition (from the top-level `shells` section) should be used for this particular command.

    * **Example**:

        ```yaml
        run_in_alpine:
          shell: docker-run # Assuming 'docker-run: docker run --rm -it {{image}} sh -c' is defined in 'shells'
          variables:
            image: alpine/git
          cmd: git clone [https://github.com/example/repo.git](https://github.com/example/repo.git)
          help: "Clones a Git repo inside an Alpine Docker container."
        ```

    * **CLI Example**: To run `run_in_alpine`:

        ```bash
        fr cmd run_in_alpine
        ```

* **`variables` (local to command)**:

    * **Purpose**: Allows you to define variables that are *specific* to this command's execution. These local variables override any global variables with the same name, but only for the duration of this command.

    * **Example**:

        ```yaml
        generate_report:
          cmd: generate-report --format {{report_format}}
          variables:
            report_format: pdf # Overrides a global 'report_format' for this command
          help: "Generates the monthly report in PDF format."
        ```

    * **CLI Example**: To run `generate_report` with its local variable:

        ```bash
        fr cmd generate_report
        ```
