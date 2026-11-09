# Local Python reference archive

`python/` contains the former Python SDK, simulation scripts, tests, integration suite,
dependency files, and original README. It is preserved locally and ignored by Git;
fresh checkouts and release packages do not contain it. Keep a separate backup if you
need historical Python reproduction. The active SDK and automated CI use Rust.

When the archive is available, run these commands from the repository root:

```sh
uv venv --python 3.12 .venv
uv pip install --python .venv/bin/python -r legacy/python/requirements.lock.txt
uv pip install --python .venv/bin/python --no-deps -e legacy/python
cargo build --locked
.venv/bin/python -m pytest -c legacy/python/pyproject.toml legacy/python/integration -q
.venv/bin/python legacy/python/scripts/sandbox.py test
```

The sandbox switches to `legacy/python/`, resolves its source and tests there, and
stores default rerun outputs locally. Shared evaluated protocols are read from `docs/`.
An explicit sandbox output directory is resolved relative to `legacy/python/`; use an
absolute path when selecting a different output location. The integration suite uses
the root `target/debug/rasqp` binary and versioned vectors in `tests/fixtures/`.

Historical reports are in `docs/reports/`, checksums in `docs/manifests/`, and the loose
original execution logs in ignored `results/logs/`. Regenerated Python reports stay in
the local archive. `python/README.md` preserves historical commands and path names;
use the commands above for the current layout. Python hypothesis generation writes a
local `python/docs/` copy rather than replacing the evaluated protocol registry.
