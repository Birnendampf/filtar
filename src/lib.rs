use pyo3::prelude::*;

/// partially create and extract tar archives, skipping specified files.
/// Only zstandard compression and decompression is supported.
#[pymodule]
mod filtar {
    use pyo3::prelude::*;
    use std::borrow::Cow;
    use std::collections::HashSet;
    use std::ffi::OsString;
    use std::fs;
    use std::hash::Hash;
    use std::io;
    use std::path::{Path, PathBuf};

    /// extract a tar file to dest while skipping files and directories from exclude
    #[pyfunction]
    #[pyo3(signature = (src, dest, exclude = None))]
    fn extract(
        py: Python,
        src: Cow<Path>,
        dest: Cow<Path>,
        exclude: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        let exclude: HashSet<PathBuf> = process_exclude(exclude)?;
        py.detach(|| {
            fs::create_dir_all(&dest)?;
            let mut skipped_dirs = Vec::new();
            let mut archive = tar::Archive::new(zstd::Decoder::new(fs::File::open(src)?)?);
            for file in archive.entries()? {
                let mut file = file?;
                let path = file.path()?;
                if exclude.contains(path.as_ref()) {
                    if file.header().entry_type().is_dir() {
                        skipped_dirs.push(path.into_owned());
                    }
                    continue;
                };
                if skipped_dirs
                    .iter()
                    .any(|excluded| path.starts_with(excluded))
                {
                    continue;
                }
                file.unpack_in(&dest)?;
            }
            Ok(())
        })
    }

    fn process_exclude<'py, K: FromPyObjectOwned<'py> + Eq + Hash>(
        exclude: Option<Bound<'py, PyAny>>,
    ) -> PyResult<HashSet<K>> {
        match exclude {
            Some(obj) => obj
                .try_iter()?
                .map(|elem| elem.and_then(|elem| elem.extract().map_err(Into::into)))
                .collect(),
            None => Ok(Default::default()),
        }
    }

    /// create a tar archive to dest while skipping files and directories from exclude
    #[pyfunction]
    #[pyo3(signature = (src, dest, n_workers = 0, level = 0, exclude = None))]
    fn create(
        py: Python,
        src: Cow<Path>,
        dest: Cow<Path>,
        n_workers: u32,
        level: i32,
        exclude: Option<Bound<PyAny>>,
    ) -> io::Result<()> {
        let exclude: HashSet<OsString> = process_exclude(exclude)?;
        py.detach(|| {
            let mut encoder = zstd::Encoder::new(fs::File::create(dest)?, level)?;
            encoder.multithread(n_workers)?;
            let mut a = tar::Builder::new(encoder);
            a.follow_symlinks(false);
            for entry in walkdir::WalkDir::new(&src)
                .min_depth(1)
                .into_iter()
                .filter_entry(|e| !exclude.contains(e.file_name()))
            {
                // XXX: some stat calls could be saved here on Windows (metadata is already
                //   populated but the append* functions call stat again)
                let entry = entry?;
                let file_type = entry.file_type();
                let path = entry.path();
                let rel_path = path.strip_prefix(&src).unwrap();
                if file_type.is_file() {
                    a.append_file(rel_path, &mut fs::File::open(path)?)?;
                } else if file_type.is_dir() {
                    a.append_dir(rel_path, path)?;
                } else if file_type.is_symlink() {
                    let mut header = tar::Header::new_gnu();
                    header.set_metadata(&entry.metadata()?);
                    a.append_link(&mut header, rel_path, fs::read_link(&path)?)?;
                } else {
                    a.append_path_with_name(path, rel_path)?;
                }
            }
            a.into_inner()?.finish()?;
            Ok(())
        })
    }
}
