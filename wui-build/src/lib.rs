#[macro_use] extern crate collect_mac;
extern crate gcc;
extern crate rustc_serialize;
extern crate tempfile;

pub use cargo::*;
pub use manifest::*;
pub use resource::*;

mod cargo;
mod manifest;
mod resource;

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

const RES_OBJ_NAME: &'static str = "wui-generated-resource";

pub fn guess() -> io::Result<()> {
    println!("wui-build: Guessing resource script...");
    let mut rc = try!(ResourceScript::guess());
    let am = try!(guess_manifest(true));
    rc.user_defined.push(UserDefined {
        name_id: CREATEPROCESS_MANIFEST_RESOURCE_ID,
        type_id: RT_MANIFEST,
        data: UserData::Data(format!("{}", am).into()),
    });

    let out_dir = try!(env::var("OUT_DIR")
        .map_err(|_| io_err("could not read OUT_DIR")));

    println!("wui-build: Guessing target...");
    let target = try!(Target::guess());

    println!("wui-build: Writing resource script...");
    let rc_path = Path::new(&out_dir)
        .join(RES_OBJ_NAME).with_extension("rc");
    let mut rc_file = try!(fs::File::create(&rc_path));
    try!(rc_file.write_all(format!("{}", rc).as_bytes()));
    drop(rc_file);

    println!("wui-build: Compiling resource script...");
    let res_path = rc_path.with_extension(target.res_ext());
    let mut rc_cmd = try!(target.rc_command(&rc_path, &res_path));
    let rc_st = try!(rc_cmd.status());

    if !rc_st.success() {
        return Err(io_err(rc_st.to_string()));
    }

    /*
    Cargo is a pain, so we have to be tricky about this.  The fundamental problem is that we can't pass arbitrary objects/options to the linker.  As a result, we have to trick Cargo into letting us link the resource object in.

    For the GNU toolchain, we do this by compiling the resource object into a static library, then link that static library into the final object.

    `LINK`, on the other hand, *will not* carry resources in a static library through to the final output, so we need to pass the resource object file directly to the linker.  Except that, as noted before, Cargo is so dogmatic that *we can't do this*.  The *only* way I could get this to work is to forcibly name the object file `*.dll`, then lie to Cargo that it's a dynamic library *and hope no one notices*.
    */
    match target.toolchain {
        Toolchain::MinGW => {
            println!("wui-build: Linking resource library...");
            gcc::Config::new()
                .object(&res_path)
                .compile(&format!("lib{}.a", RES_OBJ_NAME));
        },
        Toolchain::Msvc => {
            println!("cargo:rustc-link-lib=dylib={}", RES_OBJ_NAME);
            println!("cargo:rustc-link-search=native={}", out_dir);
        },
    }

    Ok(())
}

pub struct Target {
    target: String,
    toolchain: Toolchain,
}

pub enum Toolchain {
    MinGW,
    Msvc,
}

impl Target {
    pub fn guess() -> io::Result<Self> {
        let target = try!(env::var("TARGET")
            .map_err(|_| io_err("could not read TARGET")));

        let toolchain = if target.ends_with("-gnu") {
            Toolchain::MinGW
        } else if target.ends_with("-msvc") {
            Toolchain::Msvc
        } else {
            return Err(io_err(format!("unknown toolchain for target `{}`", target)));
        };

        Ok(Target {
            target: target,
            toolchain: toolchain,
        })
    }

    pub fn res_ext(&self) -> &'static str {
        use self::Toolchain::*;
        match self.toolchain {
            MinGW => "o",

            // Yes, I know this is wrong.  See above about cargo and `LINK`.
            Msvc => "lib",
        }
    }

    pub fn rc_command<Input, Output>(&self, input: Input, output: Output) -> io::Result<Command>
    where Input: AsRef<OsStr>, Output: AsRef<OsStr> {
        use self::Toolchain::*;
        match self.toolchain {
            MinGW => {
                let mut cmd = Command::new("windres");
                cmd.arg("-J").arg("rc")
                    .arg("-i").arg(input)
                    .arg("-O").arg("coff")
                    .arg("-o").arg(output);
                Ok(cmd)
            },
            Msvc => {
                let tool = gcc::windows_registry::find_tool(
                    &self.target, "rc.exe");
                let tool = try!(tool
                    .ok_or_else(|| io_err(format!(
                        "could not find `rc` for target `{}`",
                        self.target))));
                println!("wui-build: rc: {:?}", tool.path());
                let mut cmd = tool.to_command();
                cmd.arg("/nologo")
                    .arg("/fo").arg(output)
                    .arg(input);
                Ok(cmd)
            },
        }
    }
}

fn io_err<E>(error: E) -> ::std::io::Error
where E: Into<Box<::std::error::Error + Send + Sync>> {
    ::std::io::Error::new(::std::io::ErrorKind::Other, error)
}
