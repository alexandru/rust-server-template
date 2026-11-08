# Development

## Backend

The sample service is in `backend/main`. It prints `Hello world` and stays running until it receives SIGINT or SIGTERM.

Run the workspace quality checks with:

```sh
cd backend && cargo xtask check
```

The `Makefile` provides `make tests`, `make build`, and `make docker-build-local` from the repository root.

## Skills

The `rust-skills` skill is bundled under `.agents/skills/rust-skills/`. To update it from its recorded upstream source and refresh `skills-lock.json`, run:

```sh
make update-skills
```

This target requires Node.js 22.20 or newer and `npx`. The `code-refinements` skill referenced in `CODING_STANDARDS.md` is provided by the agent environment.

## Container image

The Docker build uses `backend/` as its build context. The publish workflow derives its GitHub Container Registry image from the repository owner and name, and publishes `amd64` and `arm64` images.
