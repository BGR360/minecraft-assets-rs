use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::api::{Error, ResourceKind, ResourceLocation, ResourcePath};

/*
 dMMMMMMP dMMMMb  .aMMMb  dMP dMMMMMMP .dMMMb
   dMP   dMP.dMP dMP"dMP amr    dMP   dMP" VP
  dMP   dMMMMK" dMMMMMP dMP    dMP    VMMMb
 dMP   dMP"AMF dMP dMP dMP    dMP   dP .dMP
dMP   dMP dMP dMP dMP dMP    dMP    VMMMP"
*/

/// Indicates that a type can enumerate available resources.
pub trait EnumerateResources {
    #[allow(missing_docs)]
    type Error;

    #[allow(missing_docs)]
    type Iter: Iterator<Item = ResourceLocation<'static>>;

    /// Enumerates the available resources of the given [`ResourceKind`] in the
    /// given namespace.
    fn enumerate_resources(
        &self,
        namespace: &str,
        kind: ResourceKind,
    ) -> Result<Self::Iter, Self::Error>;
}

/// Indicates that a type can load provide the raw data of resources.
pub trait LoadResource {
    #[allow(missing_docs)]
    type Error;

    /// Returns the raw bytes of the resource referenced by the given
    /// [`ResourceLocation`].
    fn load_resource(&self, location: &ResourceLocation) -> Result<Vec<u8>, Self::Error>;
}

/// Marker trait for types that are [`EnumerateResources`] and [`LoadResource`].
pub trait ResourceProvider: EnumerateResources + LoadResource {}

impl<T: EnumerateResources + LoadResource> ResourceProvider for T {}

/*
    dMMMMMP dMP dMP     dMMMMMP        .dMMMb  dMP dMP .dMMMb dMMMMMMP dMMMMMP dMMMMMMMMb
   dMP     amr dMP     dMP            dMP" VP dMP.dMP dMP" VP   dMP   dMP     dMP"dMP"dMP
  dMMMP   dMP dMP     dMMMP           VMMMb   VMMMMP  VMMMb    dMP   dMMMP   dMP dMP dMP
 dMP     dMP dMP     dMP            dP .dMP dA .dMP dP .dMP   dMP   dMP     dMP dMP dMP
dMP     dMP dMMMMMP dMMMMMP         VMMMP"  VMMMP"  VMMMP"   dMP   dMMMMMP dMP dMP dMP

    dMMMMb  dMMMMb  .aMMMb  dMP dMP dMP dMMMMb  dMMMMMP dMMMMb
   dMP.dMP dMP.dMP dMP"dMP dMP dMP amr dMP VMP dMP     dMP.dMP
  dMMMMP" dMMMMK" dMP dMP dMP dMP dMP dMP dMP dMMMP   dMMMMK"
 dMP     dMP"AMF dMP.aMP  YMvAP" dMP dMP.aMP dMP     dMP"AMF
dMP     dMP dMP  VMMMP"    VP"  dMP dMMMMP" dMMMMMP dMP dMP

*/

/// A [`ResourceProvider`] that provides resources from the local file system.
pub struct FileSystemResourceProvider {
    root: PathBuf,
}

impl FileSystemResourceProvider {
    /// Returns a new provider that provides resources from the given root directory.
    ///
    /// The root directory should be the directory that contains the `assets/`
    /// and (optionally) `data/` directory.
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: PathBuf::from(root.as_ref()),
        }
    }
}

impl EnumerateResources for FileSystemResourceProvider {
    type Error = Error;

    type Iter = ResourceIter;

    fn enumerate_resources(
        &self,
        namespace: &str,
        kind: ResourceKind,
    ) -> Result<Self::Iter, Self::Error> {
        let directory = ResourcePath::for_kind(&self.root, namespace, kind);
        Ok(ResourceIter::new(directory, kind)?)
    }
}

impl LoadResource for FileSystemResourceProvider {
    type Error = Error;

    fn load_resource(&self, location: &ResourceLocation) -> Result<Vec<u8>, Self::Error> {
        let path = ResourcePath::for_resource(&self.root, location);
        Ok(fs::read(path)?)
    }
}

/*
    dMP dMMMMMMP dMMMMMP dMMMMb
   amr    dMP   dMP     dMP.dMP
  dMP    dMP   dMMMP   dMMMMK"
 dMP    dMP   dMP     dMP"AMF
dMP    dMP   dMMMMMP dMP dMP

*/

/// An iterator over a directory that yields [`ResourceLocation`]s for every
/// file of a certain [`ResourceKind`].
pub struct ResourceIter {
    dir_iter: fs::ReadDir,
    kind: ResourceKind,
}

impl ResourceIter {
    pub fn new(directory: impl AsRef<Path>, kind: ResourceKind) -> Result<Self, io::Error> {
        let dir_iter = fs::read_dir(directory)?;
        Ok(Self { dir_iter, kind })
    }
}

impl Iterator for ResourceIter {
    type Item = ResourceLocation<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;

        self.dir_iter.find(|entry| {
            entry
                .as_ref()
                .ok()
                .and_then(|dir_entry| {
                    dir_entry.file_name().to_str().map(|file_name| {
                        let is_right_kind = file_name.ends_with(self.kind.extension());
                        if is_right_kind {
                            let dot_index = file_name.len() - self.kind.extension().len() - 1;
                            let location = ResourceLocation::new_owned(
                                self.kind,
                                String::from(&file_name[..dot_index]),
                            );
                            next = Some(location);
                        }
                        is_right_kind
                    })
                })
                .unwrap_or(false)
        });

        next
    }
}
