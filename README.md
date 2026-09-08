# fork-drill

A deliberately trivial repository whose only purpose is to exercise **Apex Actions'** untrusted-run
path against a real fork pull request — the live exercise owed by Phase 26, slice 1.

`trust` is decided by comparing the head repository's numeric id against the base's
(`control-plane` `src/domain/trust.ts`), so a fork owned by anyone — including a maintainer — is
untrusted. A pull request opened here from a fork must:

1. produce a run that starts **no jobs** until somebody with write access approves it;
2. once approved, run with **no repository secrets** in the job's environment;
3. hold a **read-only** token;
4. be denied **cache writes**;
5. be leased only to a runner that **belongs to a pool** — never the box.

`DRILL_SECRET` exists on this repository so that its *absence* from a fork run means something. A
`push` run on a branch of this repository is the control: it must see the secret as present.

Nothing here prints a secret's value. Presence is reported as a word, never as bytes.

## `workload-matrix` — TASK-0057

A second drill lives here, on the `workload-matrix` branch. This repository is the only place it can
run: `mail-dock` is the sanctioned test tenant, **GitHub Actions is enabled here** — it is disabled
org-wide on `Apex-Actions` — and Apex's App is installed on it too. One push therefore produces two
runs of the same commit, one on each platform, which is what makes the outputs comparable at all.

Six workloads: Node, Go, Python, Rust, a Docker build and a Postgres service, plus a job that reports
the environment facts a workflow can actually observe. Every comparable line begins `WLM|`, because a
free-text log diff is dominated by timestamps, ids and paths that are *supposed* to differ.
