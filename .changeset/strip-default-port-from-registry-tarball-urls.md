---
"pacquet": patch
---

pnpm again strips a redundant default port, such as `:443` for `https` or `:80` for `http`, from tarball URLs served by a registry before writing them to `pnpm-lock.yaml` [#15539](https://github.com/pnpm/pnpm/issues/15539).
