from _typeshed import StrPath
from collections.abc import Iterable

def extract(src: StrPath, dest: StrPath, exclude: Iterable[StrPath] | None = None) -> None: ...
def create(
    src: StrPath,
    dest: StrPath,
    exclude: Iterable[str] | None = None,
    n_workers: int = 0,
    level: int = 0,
) -> None: ...
