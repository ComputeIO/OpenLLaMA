
# Contributing

Thank you for your interest in the project and for considering contributing.

This guide should help you get started: creating a build and test environment, as well as contributing your work.

All contributions are welcome! While this guide will focus on contributing code, we would also encourage you to
contribute by reporting issues, providing feedback, suggesting new ideas. Or just by saying "hi" in the chat.

## Before you start

Before you start working on a fix or new feature, we would recommend to reach out to us and tell us about it. Maybe
we already have this in our heads (and forgot to create an issue for it), or maybe we have an alternative already.

In any case, it is always good to create an issue, or join the chat and tell us about your issues or plans. We will
definitely try to help you.

## Developing

We have some detailed development instructions in a separate document: [DEVELOPMENT.adoc](DEVELOPMENT.adoc). 

## Contributing your work

Thank you for reading the document up to this point and for taking the next step.

### Pre-flight check

Before creating a pull-request (PR), you should do some pre-flight checks, which the CI will run later on anyway.
Running locally will give you quicker results, and safe us a bit of time and CI resources.

It is as easy as running:

```shell
make check
```

This will:

* Check source code formatting
* Run `cargo check`
* Run `cargo clippy`

The `clippy` checks should be seen as *suggestions*. Take a look at them, in some cases you will learn something new. If
it sounds reasonable, it might be wise to fix it. Maybe it flags files you didn't even touch. In this case just ignore
them, was we might not have fixed all the clippy suggestions ourselves.

### Creating a PR

Nothing fancy, just a normal PR. The CI will be triggered and come back with results. People tend to pay more attention
to PRs that show up "green". So maybe check back and ensure that the CI comes up "green" for your PR as well. If it
doesn't, and you don't understand why, please reach out to us.

There are bonus points for adding your own tests ;-)

### Coding style

- Follow the instructions of [The Rust Programming Language](https://doc.rust-lang.org/book/).
- Follow [api-guidelines](https://rust-lang.github.io/api-guidelines/)
- Ensure that your code is formatted using `rustfmt` before submitting it.
- Ensure that each new source file starts with a license header.
- Ensure that there are enough unit tests.
- Ensure that there are enough comments.

### Commit message guidelines

- Follow the [Angular commit guidelines](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit).
- Commits must be signed and the signature must match the author.
- If there is an issue, you need to link to related issues.

Example:

> For example, the author information must match the Signed-off-by information.
```shell
Author: users <users@openllama.groups.io>
Date:   Thu Apr 27 09:40:02 2023 +0800

    feat(common): this is an example

    close: #1

    Signed-off-by: users <users@openllama.groups.io>
```
