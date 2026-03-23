use pyo3::prelude::*;

/// partially create and extract tar archives, skipping specified files.
/// Only zstandard compression and decompression is supported.
#[pymodule]
mod filtar {
    use pyo3::prelude::*;
    use std::borrow::Cow;
    use std::fs::File;
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
        let exclude = process_exclude(exclude)?;
        py.detach(|| {
            let mut a = tar::Archive::new(zstd::Decoder::new(File::open(src)?)?);
            for file in a.entries()? {
                let mut file = file?;
                let path = file.path()?;
                // the python implementation does not scan through all ignored files on every
                // iteration. If needed this could be optimized
                if exclude.iter().any(|excluded| path.starts_with(excluded)) {
                    continue;
                }
                file.unpack_in(&dest)?;
            }
            Ok(())
        })
    }

    fn process_exclude(exclude: Option<Bound<PyAny>>) -> PyResult<Vec<PathBuf>> {
        match exclude {
            Some(obj) => obj
                .try_iter()?
                .map(|elem| elem.and_then(|elem| elem.extract()))
                .collect(),
            None => Ok(Vec::new()),
        }
    }
}
