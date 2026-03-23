# filtar

[![PyPI - Version](https://img.shields.io/pypi/v/filtar)](https://pypi.org/project/filtar/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/filtar)]((https://pypi.org/project/filtar/))

Minimal Python extension for fast tar file creation and extraction written in Rust.

> [!IMPORTANT]
> **This module exists solely for [MineDelta](https://github.com/Birnendampf/Minedelta)** and is tailored to its needs.
> It is not a general-purpose replacement for `tarfile`.

## Constraints

* Only supports **Zstandard-compressed tar archives**
* Error handling is very bare-bones (io errors are just passed through without details)
* Behavior is intentionally limited and opinionated

## API

### `extract(src: StrPath, dest: StrPath, exclude: Iterable[StrPath] | None = None) -> None`

Extract archive to `dest`.

* `exclude`: paths to skip; matching entries and their contents (if directories) are excluded
* Creates `dest` if needed

---

### `create(src: StrPath, dest: StrPath, exclude: Set[str] = ... , n_workers: int = 0, level: int = 0) -> None`

Create archive from `src`.

* `exclude`: set of **file/directory names** to skip (not paths)
* `n_workers`: number of workers to use
* `level`: compression level
* Recursively archives contents
* Does not follow symlinks

## Notes

* `exclude` is similar to `tarfile`’s filtering, but only skips entries
* The GIL is released as much as possible
* Extraction uses **path-based exclusion**, creation uses **name-based exclusion**
* No support for other compression formats or advanced tar features

## Use

If you are not working on MineDelta, consider using `fastar` instead. It provides a more general-purpose interface to
the same underlying Rust `tar` crate, supports multiple compression formats (including transparent detection), but does
not support partial extraction/creation and does not release the GIL.
