use git2::Repository;
use moins::Moins;
use std::{env, fs, path::Path, process::Command};
use crate::{err_unrec, inf, inssort, mods::strs::succ, mods::strs::sec, mods::strs::prompt};

pub fn clone(noconfirm: bool, pkg: &str) {
    let cachedir = format!("{}/.cache/ame", std::env::var("HOME").unwrap());
    let path = Path::new(&cachedir);
    let pkgdir = format!("{}/{}", &cachedir, &pkg);
    let results = raur::search(&pkg).unwrap();

    if results.len() == 0 {
        err_unrec(format!("No matching AUR packages found"));
    }

    let url = format!("https://aur.archlinux.org/{}.git", results[0].name);

    if !path.is_dir() {
        let cache_result = fs::create_dir(&path);
        match cache_result {
        Ok(_) => {
            inf(format!("Created cache path (first run)"))
        }
        Err(_) => {
            err_unrec(format!("Could not create cache path"))
        }}
    }

    inf(format!("Cloning {} ...", pkg));

    if Path::new(&pkgdir).is_dir() {
        let rm_result = fs::remove_dir_all(&pkgdir);
        match rm_result {
        Ok(_) => {
            inf(format!("Package path for {} already found. Removing to reinstall", pkg))
        }
        Err(_) => {
            err_unrec(format!("Package path for {} already found, but could not remove to reinstall", pkg))
        }}
    }

    let dir_result = fs::create_dir(&pkgdir);
    match dir_result {
    Ok(_) => {
        inf(format!("Cloned {} to package directory", pkg))
    }
    Err(_) => {
        err_unrec(format!("Couldn't create package directory for {}", pkg))
    }}

    let cd_result = env::set_current_dir(&pkgdir);
    match cd_result {
    Ok(_) => {
        inf(format!("Entered package directory"))
    }
    Err(_) => {
        err_unrec(format!("Could not enter package directory"))
    }}

    sec(format!("Installing AUR package depends"));

    // you can use this to get the makedepends too - just use the make_depends field instead of the depends field
                                                  //     | riiiiight
    let aurpkgname = results[0].name.to_string(); //     v here
    let depends = raur::info(&[&aurpkgname]).unwrap()[0].depends.clone();

    inssort(noconfirm, depends);
    Repository::clone(&url, Path::new(&pkgdir)).unwrap();

    if noconfirm == false {
        let pkgbuild = prompt(format!("View PKGBUILD?"));

        if pkgbuild == true {
            let mut pkgbld = fs::read_to_string(format!("{}/PKGBUILD", &pkgdir)).unwrap();
            Moins::run(&mut pkgbld, None);
        }
    }

    if noconfirm == true {
        sec(format!("Installing {} ...", pkg));
        let install_result = Command::new("makepkg")
                             .arg("-si")
                             .arg("--noconfirm")
                             .status();
        match install_result {
        Ok(_) => {
            succ(format!("Succesfully installed {}", pkg));
        }
        Err(_) => {
            err_unrec(format!("Couldn't install {}", pkg));
        }};
    } else {
        sec(format!("Installing {} ...", pkg));
        let install_result = Command::new("makepkg")
                             .arg("-si")
                             .status()
                             .expect("Couldn't call makepkg");
        match install_result.code() {
        Some(0) => {
            succ(format!("Succesfully installed {}", pkg));
        }
        Some(_) => {
            err_unrec(format!("Couldn't install {}", pkg));
        }
        None => {
            err_unrec(format!("Couldn't install {}", pkg));
        }};
    }
}
