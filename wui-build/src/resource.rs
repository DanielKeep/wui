use std::collections::HashMap;
use std::fmt;
use std::io;
use ::io_err;
use ::cargo::CargoManifest;

pub struct ResourceScript {
    pub version_info: Option<VersionInfo>,
    pub user_defined: Vec<UserDefined>,
}

impl ResourceScript {
    pub fn guess() -> io::Result<ResourceScript> {
        Ok(ResourceScript {
            version_info: Some(try!(guess_version_info())),
            user_defined: vec![],
        })
    }
}

impl fmt::Display for ResourceScript {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(fmt, "#pragma code_page(65001)"));
        try!(writeln!(fmt, ""));
        if let Some(ref vi) = self.version_info {
            try!(writeln!(fmt, "1 {}", vi));
            try!(writeln!(fmt, ""));
        }
        for ud in &self.user_defined {
            try!(writeln!(fmt, "{}", ud));
            try!(writeln!(fmt, ""));
        }
        Ok(())
    }
}

pub struct UserDefined {
    pub name_id: u16,
    pub type_id: u16,
    pub data: UserData,
}

impl fmt::Display for UserDefined {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(fmt, "{} {} {}", self.name_id, self.type_id, self.data));
        Ok(())
    }
}

pub const CREATEPROCESS_MANIFEST_RESOURCE_ID: u16 = 1;
pub const RT_MANIFEST: u16 = 24;

pub enum UserData {
    Path(String),
    Data(Vec<u8>),
}

impl fmt::Display for UserData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::UserData::*;
        match *self {
            Path(ref path) => {
                try!(write!(fmt, "{:?}", path));
                Ok(())
            },
            Data(ref data) => {
                try!(writeln!(fmt, "{{"));
                for chunk in data.chunks(32) {
                    try!(write!(fmt, "  \""));
                    for b in chunk {
                        try!(write!(fmt, "\\x{:02x}", b));
                    }
                    try!(writeln!(fmt, "\""));
                }
                try!(write!(fmt, "}}"));
                Ok(())
            }
        }
    }
}

pub struct VersionInfo {
    pub file_version: Version,
    pub product_version: Version,
    pub file_flags_mask: u32,
    pub file_flags: u32,
    pub file_os: u32,
    pub file_type: u32,
    pub file_subtype: u32,

    pub string_file_info: HashMap<LocaleId, StringFileInfo>,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(fmt, "VERSIONINFO"));
        try!(writeln!(fmt, "FILEVERSION     {}", self.file_version));
        try!(writeln!(fmt, "PRODUCTVERSION  {}", self.product_version));
        try!(writeln!(fmt, "FILEFLAGSMASK   0x{:08x}", self.file_flags_mask));
        try!(writeln!(fmt, "FILEFLAGS       0x{:08x}", self.file_flags));
        try!(writeln!(fmt, "FILEOS          0x{:08x}", self.file_os));
        try!(writeln!(fmt, "FILETYPE        0x{:08x}", self.file_type));
        try!(writeln!(fmt, "FILESUBTYPE     0x{:08x}", self.file_subtype));
        try!(writeln!(fmt, "BEGIN"));
        try!(writeln!(fmt, "BLOCK \"StringFileInfo\""));
        try!(writeln!(fmt, "BEGIN"));
        for (lcid, sfi) in &self.string_file_info {
            try!(writeln!(fmt, "BLOCK \"{:08X}\"", lcid));
            try!(sfi.fmt(fmt));
        }
        try!(writeln!(fmt, "END"));
        try!(writeln!(fmt, "BLOCK \"VarFileInfo\""));
        try!(writeln!(fmt, "BEGIN"));
        for (lcid, _) in &self.string_file_info {
            try!(writeln!(fmt, "VALUE \"Translation\", 0x{:04x}, 0x{:04x}",
                lcid >> 16, lcid & 0xffff));
        }
        try!(writeln!(fmt, "END"));
        try!(writeln!(fmt, "END"));
        Ok(())
    }
}

pub struct StringFileInfo {
    pub entries: HashMap<FileInfoName, String>,
}

impl fmt::Display for StringFileInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(fmt, "BEGIN"));
        for (fin, val) in &self.entries {
            try!(writeln!(fmt, "VALUE \"{}\", \"{}\\0\"", fin.as_str(), val));
        }
        try!(writeln!(fmt, "END"));
        Ok(())
    }
}

pub fn guess_version_info() -> io::Result<VersionInfo> {
    use self::FileInfoName::*;

    let carman = try!(CargoManifest::new());
    let name = try!(carman.package_name());
    let ver_str = try!(carman.version());
    let ver = try!(parse_version(ver_str));
    let desc = try!(carman.description())
        .unwrap_or("(TODO: guess file description");
    let vi = VersionInfo {
        file_version: ver,
        product_version: ver,
        file_flags_mask: 0,
        file_flags: 0,
        file_os: VOS_NT,
        file_type: VFT_APP,
        file_subtype: VFT2_UNKNOWN,

        string_file_info: collect![
            LCID_EN_US_UTF_16 => StringFileInfo {
                entries: collect![
                    Comments => "(TODO: guess comments)".into(),
                    CompanyName => "(TODO: guess company)".into(),
                    FileDescription => desc.into(),
                    FileVersion => ver_str.into(),
                    InternalName => name.into(),
                    LegalCopyright => "(TODO: guess copyright)".into(),
                    LegalTrademarks => "(TODO: guess trademarks)".into(),
                    OriginalFilename => "(TODO: guess filename)".into(),
                    PrivateBuild => "(TODO: guess private build)".into(),
                    ProductName => name.into(),
                    ProductVersion => ver_str.into(),
                    SpecialBuild => "(TODO: guess special build)".into(),
                ],
            },
        ],
    };
    Ok(vi)
}

fn parse_version(ver: &str) -> io::Result<Version> {
    // TODO: handle "special/internal" versions somehow.
    let parts = try!(ver.split("-")
        .next()
        .ok_or_else(|| io_err("package version is empty")));
    let mut parts = parts.split(".");

    macro_rules! nvc {
        ($what:expr) => {
            try!(
                try!(parts.next()
                    .ok_or_else(|| io_err(
                        concat!("package version missing ",
                            $what,
                            " number"))
                    ))
                .parse()
                .map_err(|e| io_err(
                    format!(
                        concat!("invalid ", $what, " number: {}"),
                        e
                    )
                ))
            )
        };
    }

    let maj = nvc!("major");
    let min = nvc!("minor");
    let rev = nvc!("revision");

    /*
    There's really nothing intelligent we can use here without manually maintaining a build number somewhere.

    The components are `u16`s, which isn't enough for a timestamp.
    */
    let bld = 0;

    Ok(Version([maj, min, rev, bld]))
}

#[derive(Eq, PartialEq, Hash)]
pub enum FileInfoName {
    Comments,
    CompanyName,
    FileDescription,
    FileVersion,
    InternalName,
    LegalCopyright,
    LegalTrademarks,
    OriginalFilename,
    PrivateBuild,
    ProductName,
    ProductVersion,
    SpecialBuild,
}

impl FileInfoName {
    pub fn as_str(&self) -> &'static str {
        use self::FileInfoName::*;
        match *self {
            Comments => "Comments",
            CompanyName => "CompanyName",
            FileDescription => "FileDescription",
            FileVersion => "FileVersion",
            InternalName => "InternalName",
            LegalCopyright => "LegalCopyright",
            LegalTrademarks => "LegalTrademarks",
            OriginalFilename => "OriginalFilename",
            PrivateBuild => "PrivateBuild",
            ProductName => "ProductName",
            ProductVersion => "ProductVersion",
            SpecialBuild => "SpecialBuild",
        }
    }
}

#[derive(Copy, Clone)]
pub struct Version([u16; 4]);

impl fmt::Display for Version {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        try!(self.0[0].fmt(fmt));
        try!(",".fmt(fmt));
        try!(self.0[1].fmt(fmt));
        try!(",".fmt(fmt));
        try!(self.0[2].fmt(fmt));
        try!(",".fmt(fmt));
        try!(self.0[3].fmt(fmt));
        Ok(())
    }
}

pub const VS_VERSION_INFO: u32 = 1;

pub const VS_FF_DEBUG: u32 = 0x01;
pub const VS_FF_PRERELEASE: u32 = 0x02;
pub const VS_FF_PATCHED: u32 = 0x04;
pub const VS_FF_PRIVATEBUILD: u32 = 0x08;
pub const VS_FF_INFOINFERRED: u32 = 0x10;
pub const VS_FF_SPECIALBUILD: u32 = 0x20;

pub const VOS_UNKNOWN: u32 = 0x00000000;
pub const VOS_DOS: u32 = 0x00010000;
pub const VOS_OS216: u32 = 0x00020000;
pub const VOS_OS232: u32 = 0x00030000;
pub const VOS_NT: u32 = 0x00040000;
pub const VOS_WINCE: u32 = 0x00050000;

pub const VOS__BASE: u32 = 0x00000000;
pub const VOS__WINDOWS16: u32 = 0x00000001;
pub const VOS__PM16: u32 = 0x00000002;
pub const VOS__PM32: u32 = 0x00000003;
pub const VOS__WINDOWS32: u32 = 0x00000004;

pub const VOS_DOS_WINDOWS16: u32 = 0x00010001;
pub const VOS_DOS_WINDOWS32: u32 = 0x00010004;
pub const VOS_OS216_PM16: u32 = 0x00020002;
pub const VOS_OS232_PM32: u32 = 0x00030003;
pub const VOS_NT_WINDOWS32: u32 = 0x00040004;

pub const VFT_UNKNOWN: u32 = 0x00000000;
pub const VFT_APP: u32 = 0x00000001;
pub const VFT_DLL: u32 = 0x00000002;
pub const VFT_DRV: u32 = 0x00000003;
pub const VFT_FONT: u32 = 0x00000004;
pub const VFT_VXD: u32 = 0x00000005;
pub const VFT_STATIC_LIB: u32 = 0x00000007;

pub const VFT2_UNKNOWN: u32 = 0x00000000;
pub const VFT2_DRV_PRINTER: u32 = 0x00000001;
pub const VFT2_DRV_KEYBOARD: u32 = 0x00000002;
pub const VFT2_DRV_LANGUAGE: u32 = 0x00000003;
pub const VFT2_DRV_DISPLAY: u32 = 0x00000004;
pub const VFT2_DRV_MOUSE: u32 = 0x00000005;
pub const VFT2_DRV_NETWORK: u32 = 0x00000006;
pub const VFT2_DRV_SYSTEM: u32 = 0x00000007;
pub const VFT2_DRV_INSTALLABLE: u32 = 0x00000008;
pub const VFT2_DRV_SOUND: u32 = 0x00000009;
pub const VFT2_DRV_COMM: u32 = 0x0000000A;
pub const VFT2_DRV_INPUTMETHOD: u32 = 0x0000000B;
pub const VFT2_DRV_VERSIONED_PRINTER: u32 = 0x0000000C;

pub const VFT2_FONT_RASTER: u32 = 0x00000001;
pub const VFT2_FONT_VECTOR: u32 = 0x00000002;
pub const VFT2_FONT_TRUETYPE: u32 = 0x00000003;

pub const LCID_EN_US_UTF_16: u32 = 0x040904B0;
pub const LCID_EN_US_WINDOWS_1252: u32 = 0x040904E4;

pub type LocaleId = u32;
