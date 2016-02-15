use std::io;
use std::process::Command;
use rustc_serialize::json::{self, Json};
use ::io_err;

pub struct CargoManifest {
    manifest: Json,
}

impl CargoManifest {
    pub fn new() -> io::Result<Self> {
        Ok(CargoManifest {
            manifest: try!(cargo_read_manifest()),
        })
    }

    pub fn package_name(&self) -> io::Result<&str> {
        let manifest = &self.manifest;
        let name = try!(manifest.find_path(&["name"])
            .ok_or_else(|| io_err("could not find package name in manifest")));
        name.as_string()
            .ok_or_else(|| io_err("package name is not a string"))
    }

    pub fn description(&self) -> io::Result<Option<&str>> {
        // NOTE: this is not included in the output of `read-manifest`.
        Ok(None)
    }

    pub fn version(&self) -> io::Result<&str> {
        try!(self.manifest.find_path(&["version"])
            .ok_or_else(|| io_err("could not find package version in manifest")))
            .as_string()
            .ok_or_else(|| io_err("package version is not a string"))
    }
}

fn cargo_read_manifest() -> io::Result<json::Json> {
    let output = try!(Command::new("cargo")
        .arg("read-manifest")
        .output());

    if !output.status.success() {
        return Err(io_err("error running `cargo read-manifest`"));
    }

    match json::Json::from_reader(&mut &*output.stdout) {
        Ok(json) => Ok(json),
        Err(err) => Err(io_err(err)),
    }
}
