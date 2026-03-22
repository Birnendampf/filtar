use pyo3::prelude::*;

/// partially create and extract tar archives, skipping specified files.
/// Only zstandard compression and decompression is supported.
#[pymodule]
mod filtar {
    use pyo3::prelude::*;
    use std::fs::File;
    use std::path::PathBuf;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    #[pyo3(signature = (src, dest, exclude = None))]
    fn extract(
        py: Python,
        src: PathBuf,
        dest: PathBuf,
        exclude: Option<Bound<PyAny>>,
    ) -> PyResult<()> {
        let exclude = match exclude {
            Some(obj) => obj
                .try_iter()?
                .map(|elem| elem.and_then(|elem| elem.extract()))
                .collect::<PyResult<Box<[PathBuf]>>>()?,
            None => Box::default(),
        };
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
}
