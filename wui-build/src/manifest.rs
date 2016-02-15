use std::borrow::Cow;
use std::fmt;
use std::io;
use std::io::Write;
use ::cargo::CargoManifest;

type CowStr = Cow<'static, str>;

pub const XMLNS_ASM_V1: &'static str = "urn:schemas-microsoft-com:asm.v1";

pub const ASM_COMCTL6: AssemblyIdent = AssemblyIdent {
    name: Cow::Borrowed("Microsoft.Windows.Common-Controls"),
    version: Cow::Borrowed("6.0.0.0"),
    proc_arch: None,
    lang: None,
    public_key: Some(Cow::Borrowed("6595b64144ccf1df")),
};

#[derive(Clone, Debug)]
pub struct Assembly {
    identity: AssemblyIdent,
    dependencies: Cow<'static, [AssemblyIdent]>,
}

#[derive(Clone, Debug)]
pub struct AssemblyIdent {
    name: CowStr,
    version: CowStr,
    proc_arch: Option<ProcArch>,
    lang: Option<CowStr>,
    public_key: Option<CowStr>,
}

#[derive(Copy, Clone, Debug)]
pub enum ProcArch {
    X86,
    Amd64,
    Ia64,
}

pub fn quick_manifest(out: &mut Write, use_comctl6: bool) -> io::Result<()> {
    let manifest = try!(guess_manifest(use_comctl6));
    write_manifest(&manifest, out)
}

pub fn guess_manifest(use_comctl6: bool) -> io::Result<Assembly> {
    use std::env;

    let carman = CargoManifest::new().unwrap();
    let name = carman.package_name().unwrap().to_owned();
    let version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION not set");
    let version = format!("{}.0", version);

    let target = env::var("TARGET").expect("TARGET not set");
    let proc_arch = if target.starts_with("i686-") {
        ProcArch::X86
    } else if target.starts_with("x86_64-") {
        ProcArch::Amd64
    } else {
        panic!("Unrecognised target: `{}`", target);
    };

    Ok(Assembly {
        identity: AssemblyIdent {
            name: name.into(),
            version: version.into(),
            proc_arch: Some(proc_arch),
            lang: None,
            public_key: None,
        },
        dependencies: {
            if use_comctl6 {
                const COMCTL6_DEPS: &'static [AssemblyIdent] = &[ASM_COMCTL6];
                COMCTL6_DEPS.into()
            } else {
                [][..].into()
            }
        },
    })
}

pub fn write_manifest(assembly: &Assembly, out: &mut Write) -> io::Result<()> {
    try!(write!(out, "<?xml version='1.0' encoding='UTF-8' standalone='yes'?>\r\n"));
    try!(write!(out, "{}", assembly));
    Ok(())
}

impl fmt::Display for Assembly {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::xml_escape_attr as xea;
        try!(write!(fmt,
            "<assembly xmlns='{xmlns}' manifestVersion='1.0'>",
            xmlns = xea(XMLNS_ASM_V1),
        ));
        try!(write!(fmt, "{}", self.identity));
        if self.dependencies.len() > 0 {
            try!(write!(fmt, "<dependency>"));
            for dep in &*self.dependencies {
                try!(write!(fmt, "<dependentAssembly>"));
                try!(write!(fmt, "{}", dep));
                try!(write!(fmt, "</dependentAssembly>"));
            }
            try!(write!(fmt, "</dependency>"));
        }
        try!(write!(fmt, "</assembly>"));
        Ok(())
    }
}

impl fmt::Display for AssemblyIdent {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::xml_escape_attr as xea;
        try!(write!(fmt,
            "<assemblyIdentity \
                type='win32' \
                name='{name}' \
                version='{version}' \
                processorArchitecture='{proc_arch}' \
            ",
            name = xea(&self.name),
            version = xea(&self.version),
            proc_arch = DisplayOr::some_or(self.proc_arch.as_ref(), "*"),
        ));
        if let Some(ref lang) = self.lang {
            try!(write!(fmt, " language='{}'", lang));
        }
        if let Some(ref public_key) = self.public_key {
            try!(write!(fmt, " publicKeyToken='{}'", public_key));
        }
        try!(write!(fmt, "/>"));
        Ok(())
    }
}

impl fmt::Display for ProcArch {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::ProcArch::*;
        match *self {
            X86 => fmt.write_str("x86"),
            Amd64 => fmt.write_str("amd64"),
            Ia64 => fmt.write_str("ia64"),
        }
    }
}

enum DisplayOr<A: fmt::Display, B: fmt::Display> {
    Some(A),
    None(B),
}

impl<A, B> DisplayOr<A, B>
where A: fmt::Display, B: fmt::Display {
    pub fn some_or(opt: Option<A>, or: B) -> Self {
        match opt {
            Some(a) => DisplayOr::Some(a),
            None => DisplayOr::None(or),
        }
    }
}

impl<A, B> fmt::Display for DisplayOr<A, B>
where A: fmt::Display, B: fmt::Display {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::DisplayOr::*;
        match *self {
            Some(ref a) => fmt::Display::fmt(a, fmt),
            None(ref b) => fmt::Display::fmt(b, fmt),
        }
    }
}

fn xml_escape_attr(s: &str) -> Cow<str> {
    if !(s.contains('\'') || s.contains('"') || s.contains('&')) {
        Cow::Borrowed(s)
    } else {
        let cap = s.bytes()
            .map(|b| match b {
                b'\'' => "&apos;".len(),
                b'"' => "&quot;".len(),
                b'&' => "&amp;".len(),
                _ => 1
            })
            .fold(0, |a, b| a + b);
        let mut r = String::with_capacity(cap);
        for c in s.chars() {
            match c {
                '\'' => r.push_str("&apos;"),
                '"' => r.push_str("&quot;"),
                '&' => r.push_str("&amp;"),
                c => r.push(c)
            }
        }
        Cow::Owned(r)
    }
}

#[cfg(test)]
#[test]
fn test_xml_escape_attr() {
    use self::xml_escape_attr as xea;
    assert_eq!(xea("abcd"), "abcd");
    assert_eq!(xea("ab'cd"), "ab&apos;cd");
    assert_eq!(xea("ab\"cd"), "ab&quot;cd");
    assert_eq!(xea("ab&cd"), "ab&amp;cd");
    assert_eq!(xea("ab&apos;cd"), "ab&amp;apos;cd");
}
