extern crate toml;
extern crate rustc_serialize;
extern crate rpf;
extern crate tar;
extern crate time;
extern crate walkdir;

use ext::{parse_toml_file,Splits};

use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::fs::File;
use std::io::prelude::*;

use rpf::*;
use time::*;
use tar::Archive;
use rustc_serialize::{Encoder,Encodable};
use rustc_serialize::json;
use walkdir::WalkDir;

// Structure for describing a package to be built
#[derive(RustcDecodable,RustcEncodable,Debug,Default,PartialEq)]
pub struct BuildFile {
    name: Option<String>,
    vers: Option<String>,
    build: Option<Vec<String>>,
    builddate: Option<String>,
    desc: Option<String>,
    prefix: Option<String>,
    package: Option<Vec<String>>,
    makedeps: Option<Vec<String>>,
    deps: Option<Vec<String>>,
    arch: Option<String>,
    url: Option<String>,
    source: Option<String>,
    license: Option<String>,
    provides: Option<String>,
    conflicts: Option<Vec<String>>,
    maintainers: Option<Vec<String>>,
}

impl BuildFile {
    // Creates a new blank BuldFile
    pub fn new() -> BuildFile {
        let mut bf: BuildFile = Default::default();
        bf.builddate = Some(time::strftime("%m%d%Y%H%M%S", &time::now()).unwrap());
        return bf;
    }

    // Creates a BuldFile struct from a TOML file
    pub fn from_file(file: &str) -> Result<BuildFile, json::DecoderError> {
        match json::decode(&parse_toml_file(file).to_string()) {
            Ok(k) => { return Ok(k) },
            Err(e) => { return Err(e) }
        }
    }

    // Prints the BuildFile's serialized TOML as pretty JSON
    pub fn print_json(&self) {
        println!("{}", json::as_pretty_json(&self));
    }

    pub fn set_builddate(&mut self) {
        self.builddate = Some(time::strftime("%m%d%Y%H%M%S", &time::now()).unwrap());
    }
}

// Trait for performing a build
pub trait Builder {
    // Creates a package from tar file
    fn create_pkg(&mut self) -> io::Result<()>;
    // Creates a tar file
    fn create_tar_file(&self) -> Result<File, io::Error>;
    // Sets the build environment for the package
    fn set_env(&self) -> io::Result<()>;
    // Builds pacakge
    fn build(&self) -> io::Result<()>;
    // Strips 'build' from paths. This **should** be temporary
    fn strip_parent(&self, path: PathBuf) -> PathBuf;
}

impl Builder for BuildFile {
    // Strip 'build' from path's as using this directory is currently hard coded
    // behaviour
    fn strip_parent(&self, path: PathBuf) -> PathBuf {
        let mut new_path = PathBuf::new();
        for component in path.components() {
            if component.as_ref() != "build" {
                new_path.push(component.as_ref());
            }
        }
        new_path
    }

    // 'touches' a tarball file using the string in 'name' as a file name
    fn create_tar_file(&self) -> Result<File, io::Error> {
        let mut tar_name = self.name.clone().unwrap();
        tar_name.push_str(".pkg.tar");
        match File::create(&tar_name) {
            Ok(s) => { return Ok(s) },
            Err(e) => { return Err(e) }
        };
    }

    // Creates a package tarball
    fn create_pkg(&mut self) -> io::Result<()> {
        let tar = try!(self.create_tar_file());
        let archive = Archive::new(tar);
        self.set_builddate();
        for entry in WalkDir::new("build") {
            let entry = try!(entry);
            let file_name = self.strip_parent(entry.path().to_path_buf());
            let metadata = try!(fs::metadata(entry.path()));
            if metadata.is_file() {
                let mut file = try!(File::open(entry.path()));
                try!(archive.append_file(file_name, &mut file));
            } else if metadata.is_dir() {
                if entry.path() != "build".as_path() {
                    try!(archive.append_dir(file_name, entry.path()));
                }
            }
        }
        // Wrap this turd up
        match archive.finish() {
            Ok(_) => {
                println!("{}: package '{}' successfully built", "mpm", &self.name.clone().unwrap());
                return Ok(());
            },
            Err(e) => { return Err(e) }
        }
    }

    // This should ideally create a build environment from PKG.toml
    fn set_env(&self) -> io::Result<()> {
        unimplemented!();
    }

    fn build(&self) -> io::Result<()> {
        for line in self.build.clone().unwrap() {
            let parsed_line: Vec<&str> = line.split(' ').collect();
            match parsed_line.split_frst() {
                Some(s) => {
                    let command = try!(Command::new(s.0).args(s.1).output());
                    println!("{}", String::from_utf8_lossy(&command.stdout));
                },
                None => { () },
            };
        }
        return Ok(());
    }
}

#[derive(RustcDecodable,RustcEncodable,Debug,Default,PartialEq)]
pub struct PkgInfo {
    name: String,
    vers: String,
    builddate: String,
    url: String,
    size: u64,
    arch: String,
    license: String,
    conflicts: Vec<String>,
    provides: String,
}

impl PkgInfo {
    pub fn new(build_file: &BuildFile) -> PkgInfo {
        let mut info: PkgInfo = Default::default();
        info.name = build_file.name.clone().unwrap();
        info.builddate = time::strftime("%m%d%Y%H%M%S", &time::now()).unwrap();
        return info;
    }
}

#[test]
fn test_new_empty_build_file() {
    let build_file = BuildFile::new();
    assert_eq!(build_file.name, None);
    assert_eq!(build_file.vers, None);
    assert_eq!(build_file.build, None);
}

#[test]
fn test_from_file() {
    let build_file = match BuildFile::from_file("example/PKG.toml") {
        Ok(s) => { s },
        Err(_) => { panic!() }
    };
    assert_eq!(build_file.name.unwrap(), "hello-mpm".to_owned());
}

#[test]
fn test_print_json() {
    let build_file = BuildFile::from_file("example/PKG.toml").unwrap();
    build_file.print_json();
}

#[test]
fn test_new_empty_pkginfo() {
    let mut build_file = BuildFile::new();
    build_file.name = Some("test".to_owned());
    let info = PkgInfo::new(&build_file);
}