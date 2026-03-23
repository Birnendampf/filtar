from _typeshed import StrPath
from collections.abc import Iterable, Set


def extract(src: StrPath, dest: StrPath, exclude: Iterable[StrPath] | None = None) -> None: ...


def create(src: StrPath, dest: StrPath, exclude: Set[str] = ...) -> None: ...
