# Contribution guidelines
I'm grateful that you're considering contributing to my project. This will be a short read to get you familiar with this repository.

**Please create an issue before starting work on your PRs!**

## Developer environment
`srt-linter` is written in Rust (currently built against `1.86.0`) so you need to setup `cargo` and friends ([look here](https://www.rust-lang.org/tools/install)). You can use whatever IDE/editor you want.

```bash
$ git clone git@github.com/furtidev/srt-linter # ssh (rec)
$ git clone https://github.com/furtidev/srt-linter # https
$ cd ./srt-linter
$ cargo build # a clean build
```

Now, you're ready to make changes!

## Commit messages
To keep things orgazined, please format your commit messages accordingly to the graph below.

```
<type>(optional scope): <description>                  
  │    │                 │                             
  │    │                 └─►a short description of     
  │    │                    the changes, all lower case
  │    │                                               
  │    └─► e.g: lexer, parser, tui, cli                
  │                                                    
  └──────►feat | fix | docs | refactor | ci | tests | chore
```

Example: `fix(lexer): don't generate duplicate tokens`

Commit types:
- `feat`: A new feature/addition.
- `fix`: A bug fix.
- `docs`: Change in documentation.
- `refactor`: Changes in code that does not alter behavior.
- `ci`: Changes in CI/CD pipeline.
- `tests`: Fixing or adding tests.
- `chore`: Anything that doesn't apply to the ones above. E.g: README changes.

## Things to do before sending the PR
CI/CD will check for these anyway and complain, but it's better to not get screamed at.
```bash
$ cargo fmt --all # format the codebase
$ cargo clippy # make sure clippy is happy :3
$ cargo test # make you haven't broken anything!
```

If all went well, you're ready to send the PR.