use crate::ffi;
use std::path::Path;

/// Flag values for the disk image format type.  Each type has a
/// bit associated with it.  There are TSK_IMG_TYPE_ISXXX macros
/// to determine the broad group of the type (raw vs aff etc.)
pub enum ImgType {
    ///< Use autodetection methods
    Detect = 0x0000,
    ///< Raw disk image (single or split)
    Raw = 0x0001,
    ///< AFF AFF Format
    Aff = 0x0004,
    ///< AFD AFF Format
    Afd = 0x0008,
    ///< AFM AFF Format
    Afm = 0x0010,
    ///< Any format supported by AFFLIB (including beta ones)
    AffAny = 0x0020,
    ///< EWF version
    Ewf = 0x0040,
    ///< VMDK version
    Vmdk = 0x0080,
    ///< VHD version
    Vhd = 0x0100,
    ///< external defined format which at least implements TSK_IMG_INFO, used by pytsk
    External = 0x1000,
    ///< Pool
    Pool = 0x4000,
    ///< Unsupported disk image type
    Unsupported = 0xffff,
}

pub struct ImgInfo {
    inner: ffi::TSK_IMG_INFO,
}

impl ImgInfo {
    pub fn open<P: AsRef<Path>>(path: P, img_type: ImgType, size: usize) -> Result<Self, ()> {
        unimplemented!();
    }

    pub fn close(&mut self) {
        unimplemented!();
    }
}
