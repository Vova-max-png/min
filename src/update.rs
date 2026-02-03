use self_update::cargo_crate_version;

use crate::AsyncError;

pub struct Updater;

pub enum UpdateConfig {
    AutoUpdate,
    DoNotUpdate,
}

impl Updater {
    pub fn new(config: UpdateConfig) -> Self {
        Self
    }

    pub fn update(&self) -> Result<(), Box<AsyncError>> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("Vova-max-png")
            .repo_name("min")
            .bin_name("min-x86_64-pc-windows-msvc.exe")
            .show_download_progress(true)
            .current_version(cargo_crate_version!())
            .build()?
            .update()?;
        println!("Update status: {:#?}", status);

        // let releases = self_update::backends::github::ReleaseList::configure()
        //     .repo_owner("Vova-max-png")
        //     .repo_name("min")
        //     .build()
        //     .unwrap()
        //     .fetch()
        //     .unwrap();
        // if releases.is_empty() {
        //     return Ok(());
        // }

        // let asset = releases[0]
        //     .asset_for(&self_update::get_target(), None)
        //     .unwrap();

        // let tmp_dir = tempfile::Builder::new()
        //     .prefix("update")
        //     .tempdir_in(::std::env::current_dir().unwrap())
        //     .unwrap();
        // let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        // let tmp_tarball = ::std::fs::File::create_new(&tmp_tarball_path).unwrap();
        // // pause();

        // self_update::Download::from_url(&asset.download_url)
        //     .set_header(
        //         reqwest::header::ACCEPT,
        //         "application/octet-stream".parse().unwrap(),
        //     )
        //     .download_to(&tmp_tarball)
        //     .unwrap();

        // let bin_name = std::path::PathBuf::from("min-x86_64-pc-windows-msvc.exe");
        // self_update::Extract::from_source(&tmp_tarball_path)
        //     .archive(self_update::ArchiveKind::Zip)
        //     .extract_file(&tmp_dir.path(), &bin_name)
        //     .unwrap();

        // let new_exe = tmp_dir.path().join(bin_name);
        // self_replace::self_replace(new_exe).unwrap();

        Ok(())
    }
}
