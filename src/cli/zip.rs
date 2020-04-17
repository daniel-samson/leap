pub fn extract(path: &std::path::Path, extract_to: &std::path::Path) -> Result<(), String> {
    let file = std::fs::File::open(&path).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = extract_to.join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                log::info!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            log::info!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            std::fs::create_dir_all(&outpath).unwrap();
        } else {
            log::info!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = std::fs::File::create(&outpath).unwrap();
            std::io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    Ok(())
}
