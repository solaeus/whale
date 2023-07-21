# Whale

Whale is data-oriented programming language for system management tasks such as
installing packages, managing disks and getting system information. Whale comes
with a command-line tool and an interactive shell that provides a live REPL with
syntax completion, history and highlighting.

The most basic whale program:

```rust
output "Hello world!"
```

Run two files at the same time:

```rust
run::async ("foo.whale", "bar.whale")
```

Wait for a file to change, then print the time:

```rust
wait watch "foo.whale";
output "The time is " + date::time;
```

## Feautures

- Data-oriented
- Declarative and explicit
- Small and easy to learn
- Manage immutable operating systems
- Extensive features for containerized workflows

## Motivation

Using the command line is like speaking a language: you learn idioms and develop
preferences based on how you use it. The hard part is knowing *how* to say
what you want. Let's say we want to partition a hard drive. We need to know
the device path, label type, partition name, filesystem type and byte range.
With whale, we simply describe the desired state in a file and let one of the
language's functions do the rest.

```rust
path = "/dev/sda";
label = "gpt";
name = "partition1";
filesystem = "ext4";
range = ("1Mib", "8MiB");
```

Then run the function with the name of the file.

```rust
disk::partition "my_partition.whale"
```

Here's the same code using a conventional shell with GNU's parted.

```sh
parted /dev/sda mklabel gpt mkpart partition1 ext4 1MiB 8MiB 
```

It fits nicely on one line, but after a few months you may want to make a change
and you will wish it was more explicit. The traditional Unix command has only
values, but whale has key-value pairs. The names of the keys are easier to
remember than how to order the information.

## Implementation

Whale serves its use cases in the easiest and most obvious way possible. It
implements solutions with the same philosophy: the `disk::partition` function
uses GNU's parted, acting as a simple API layer when possible. Whale always uses
[fish] as a shell when running commands. This is because other shells do not
support user-modified commands in non-interactive mode.

Whale is highly focused on serving users of immutable operating systems and
containerized workflows. While it works an any system, it has advanced features
such as layering packages on the base OS and package management inside the
container.

Whale is a hard fork of [evalexpr]. Thanks to the creator and contributors of
that project.

[evalexpr]: https://github.com/ISibboI/evalexpr
[toolbox]: https://containertoolbx.org
[fish]: https://fishshell.com
