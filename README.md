# `stack`

## Overview

`stack` is a command-line tool for managing stacked diffs in Git repositories, inspired by Henry Ford's revolutionary 
assembly line. It brings seamless automation to your GitHub stacks, allowing you to streamline, iterate, and ship 
faster. Like an assembly line, `stack` breaks down large code changes into smaller, manageable units that flow smoothly 
through your development process.

This project is in its early stages, and we are actively seeking contributors to shape its development.

## What Are Stacked Diffs?

Stacked diffs, also known as stacked changes or stacked PRs, are like an assembly line for your code. Instead of 
building one large feature branch, changes are divided into a sequence of dependent branches, each representing a 
logical step towards the final implementation. Each branch builds upon the previous one, forming a directed acyclic 
graph (DAG) of changes.

This approach offers several advantages:

- **Efficient Production:** Like Ford's assembly line, each PR is a specialized station in your development process.
- **Quality Control:** Small, incremental PRs are easier to review and test, ensuring high-quality output.
- **Parallel Processing:** Teams can work on different parts of a feature simultaneously, maximizing productivity.

## Why `stack`?

`stack` is designed as a lightweight, Git-native tool that brings assembly line efficiency to your development workflow. 
The key differentiators are:

1. **No Centralized Service**
   `stack` operates without requiring a remote server or database. All necessary information is stored directly in PR 
   descriptions, allowing full stack synchronization across all team members without additional infrastructure.

2. **Works with GitHub and GitLab PRs**
   Like Ford's universal assembly line concept, this tool works with multiple Git hosting providers. Any PR-based 
   workflow can be used without modifying existing development processes.

3. **Minimal Local State**
   `stack` retrieves stack metadata dynamically from PRs rather than requiring all branches to be checked out locally. 
   This makes it easy to switch between repositories and work across multiple projects without maintaining a persistent 
   state.

4. **Lean and Automated Workflow**
   The CLI is designed to automate common stacking tasks while keeping a simple interface. Commands like `move` and 
   `ship` streamline reordering and pushing stacked PRs without requiring manual intervention.

## Installation

`stack` is in early development. Once the first release is available, installation instructions will be provided for 
different platforms.

## Usage
The following commands outline the core functionality of `stack`:

### Basic Operations

```sh
# Initialize a new stack
stack init

# Sync the current state of all stacks
stack sync

# Switch to a different branch in the stack interactivly
stack switch

# Switch to a specific branch in the stack
stack switch <branch-name>
```

### Stack Navigation

```sh
# Move up one level in the stack
stack up

# Move down one level in the stack
stack down

# Restack the current branch and its children
stack restack
```

### Stack Management

```sh
# View the stack history
stack log

# Submit changes to the remote repository
stack submit
```

## Development and Contributions

`stack` is an open-source project in its early stages. Contributions are welcome, whether through issue discussions, 
feature proposals, or pull requests.

### Roadmap

- Initial CLI implementation with basic stack management
- GitHub and GitLab API integration for PR metadata storage
- Improved interactive stack visualization
- Expanded automation for stack rebasing and merging

## License

`stack` is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Get Involved

If you are interested in bringing assembly line efficiency to your development process and want to help define the 
future of `stack`, join the discussions in the issue tracker and contribute to the repository.
