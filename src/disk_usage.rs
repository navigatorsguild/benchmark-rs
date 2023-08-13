use std::io::ErrorKind;
use std::path::PathBuf;

/// Measure disk usage for a path
#[allow(dead_code)]
pub fn disk_usage(path: &PathBuf) -> std::io::Result<u64> {
    if !path.exists() {
        Err(std::io::Error::new(
            ErrorKind::Other,
            format!("path does not exist: [{}]", path.to_string_lossy()),
        ))
    } else {
        let mut size: u64 = 0;
        if path.is_dir() {
            let dir = std::fs::read_dir(path)?;
            for result in dir.into_iter() {
                let entry = result.unwrap();
                size += disk_usage(&entry.path())?;
            }
        } else if path.is_file() {
            size += path.metadata().unwrap().len();
        } else {
            // ignore
        }

        Ok(size)
    }
}

/// Convert disk usage to human format
#[allow(dead_code)]
pub fn to_human(size: u64) -> String {
    if size / 0x10000000000_u64 > 0 {
        format!("{:.3}T", size as f64 / 0x10000000000_u64 as f64)
    } else if size / 0x40000000_u64 > 0 {
        format!("{:.3}G", size as f64 / 0x40000000_u64 as f64)
    } else if size / 0x100000_u64 > 0 {
        format!("{:.3}M", size as f64 / 0x100000_u64 as f64)
    } else if size / 0x400_u64 > 0 {
        format!("{:.3}K", size as f64 / 0x400_u64 as f64)
    } else {
        format!("{}", size)
    }
}

#[cfg(test)]
mod tests {
    use crate::disk_usage::{disk_usage, to_human};
    use std::path::PathBuf;

    #[test]
    fn test_disk_usage() -> std::io::Result<()> {
        assert_eq!(
            to_human(disk_usage(&PathBuf::from("./tests/fixtures/1.5K/512"))?),
            "512".to_string()
        );
        assert_eq!(
            to_human(disk_usage(&PathBuf::from("./tests/fixtures/1.5K/1K"))?),
            "1.000K".to_string()
        );
        assert_eq!(
            to_human(disk_usage(&PathBuf::from("./tests/fixtures/1.5K/"))?),
            "1.500K".to_string()
        );
        Ok(())
    }
}
