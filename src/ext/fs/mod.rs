use std::{borrow::Cow, path::Path};

use super::{web::PermissionsContainer, ExtensionTrait};
use deno_core::{extension, Extension};
use deno_fs::FileSystemRc;
use deno_io::fs::FsError;
use deno_permissions::PermissionCheckError;

extension!(
    init_fs,
    deps = [rustyscript],
    esm_entry_point = "ext:init_fs/init_fs.js",
    esm = [ dir "src/ext/fs", "init_fs.js" ],
);
impl ExtensionTrait<()> for init_fs {
    fn init((): ()) -> Extension {
        init_fs::init()
    }
}
impl ExtensionTrait<FileSystemRc> for deno_fs::deno_fs {
    fn init(fs: FileSystemRc) -> Extension {
        deno_fs::deno_fs::init::<PermissionsContainer>(fs)
    }
}

pub fn extensions(fs: FileSystemRc, is_snapshot: bool) -> Vec<Extension> {
    vec![
        deno_fs::deno_fs::build(fs, is_snapshot),
        init_fs::build((), is_snapshot),
    ]
}

impl deno_fs::FsPermissions for PermissionsContainer {
    fn check_open<'a>(
        &mut self,
        read: bool,
        write: bool,
        path: std::borrow::Cow<'a, Path>,
        api_name: &str,
        get_path: &'a dyn deno_fs::GetPath,
    ) -> Result<deno_fs::CheckedPath<'a>, FsError> {
        let (needs_canonicalize, normalized_path_cow) = get_path.normalized(path)?;

        if needs_canonicalize {
            let resolved_path_buf = get_path.resolved(&normalized_path_cow)?;

            let p = self
                .0
                .check_open(
                    true,
                    read,
                    write,
                    Cow::Borrowed(&resolved_path_buf),
                    api_name,
                )
                .ok_or(FsError::NotCapable("Access Denied"))?;

            Ok(deno_fs::CheckedPath::Resolved(Cow::Owned(p.into_owned())))
        } else {
            let p = self
                .0
                .check_open(false, read, write, normalized_path_cow, api_name)
                .ok_or(FsError::NotCapable("Access Denied"))?;

            Ok(deno_fs::CheckedPath::Unresolved(p))
        }
    }

    fn check_read(
        &mut self,
        path: &str,
        api_name: &str,
    ) -> Result<std::path::PathBuf, PermissionCheckError> {
        self.0.check_read_all(Some(api_name))?;
        let p = self
            .0
            .check_read(Cow::Borrowed(Path::new(path)), Some(api_name))
            .map(std::borrow::Cow::into_owned)?;
        Ok(p)
    }

    fn check_read_path<'a>(
        &mut self,
        path: Cow<'a, std::path::Path>,
        api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, PermissionCheckError> {
        self.0.check_read_all(Some(api_name))?;
        let p = self.0.check_read(path, Some(api_name))?;
        Ok(p)
    }

    fn check_read_all(&mut self, api_name: &str) -> Result<(), PermissionCheckError> {
        self.0.check_read_all(Some(api_name))?;
        Ok(())
    }

    fn check_read_blind(
        &mut self,
        p: &std::path::Path,
        display: &str,
        api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        self.0.check_read_all(Some(api_name))?;
        self.0.check_read_blind(p, display, api_name)?;
        Ok(())
    }

    fn check_write(
        &mut self,
        path: &str,
        api_name: &str,
    ) -> Result<std::path::PathBuf, PermissionCheckError> {
        self.0.check_write_all(api_name)?;
        let p = self
            .0
            .check_write(Cow::Borrowed(Path::new(path)), Some(api_name))
            .map(std::borrow::Cow::into_owned)?;
        Ok(p)
    }

    fn check_write_path<'a>(
        &mut self,
        path: Cow<'a, std::path::Path>,
        api_name: &str,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, PermissionCheckError> {
        self.0.check_write_all(api_name)?;
        let p = self.0.check_write(path, Some(api_name))?;
        Ok(p)
    }

    fn check_write_partial(
        &mut self,
        path: &str,
        api_name: &str,
    ) -> Result<std::path::PathBuf, PermissionCheckError> {
        self.0.check_write_all(api_name)?;
        let p = self.0.check_write_partial(path, api_name)?;
        Ok(p)
    }

    fn check_write_all(&mut self, api_name: &str) -> Result<(), PermissionCheckError> {
        self.0.check_write_all(api_name)?;
        Ok(())
    }

    fn check_write_blind(
        &mut self,
        p: &std::path::Path,
        display: &str,
        api_name: &str,
    ) -> Result<(), PermissionCheckError> {
        self.0.check_write_all(api_name)?;
        self.0.check_write_blind(p, display, api_name)?;
        Ok(())
    }
}
