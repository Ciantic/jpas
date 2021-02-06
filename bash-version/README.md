# jpas open and save as bash scripts

See the usage and idea from the [main README](../README.md), the `jpas-open.sh`
is equivalent to `jpas open`.

Generally I don't like bash scripts, but short scripts are easier to verify and
understand than binaries with plenty of dependencies. They also work as an
escape hatch in case the large single binary aproach start to stink.

## Bash guidelines

-   Only the core behavior is meant to be implemented as bash scripts.
-   Bash scripts with no functions or other shenanigans, they are readable from
    top to down.
-   Bash scripts should not depend on other bash scripts.
-   Bash scripts should depend only on few known tools, currently `jq`, `gpg`.
-   I use shellcheck but it's just VSCode extension's defaults what ever those are.
