
pub fn main() {
    let os = "mac"; // Set a OS
    let subsys = ""; // Only for linux
    let mut de = ""; // A Desktop Enviroment
    let art = ""; // Overides Art

    // ART
    let mut _mac = false;

    let mut macartln1 = "";
    let mut macartln2 = "";
    let mut macartln3: &'static str = "";
    let mut linuxln1 = "";
    let mut linuxln2 = "";
    let mut linuxln3 = "";

    macartln1 = "     ";
    macartln2 = "     ";
    macartln3 = "     ";

    linuxln1 = "     ";
    linuxln2 = "     ";
    linuxln3 = "     ";

    if art != "" {
        if art == "mac" {_mac = true;}
    }

    if os == "" {
        exit();
    }

    if os == "linux" {
        let mut _arch = false;
        let mut _ubuntu: bool = false;
        let mut _mint: bool = false;
        let mut _linux: bool = false;
        
        if subsys == "arch" {_arch = true;}
        if subsys == "ubuntu" {_ubuntu = true;}
        if subsys == "mint" {_mint = true;}
        if subsys == "" {_linux = true;}
    }

    if os == "mac" {
        de = "MacOS Desktop Enviroment";
    }

    // Print
    if _mac == true {
        println!("{} {}",
        macartln1, os
        );
        println!("{}",
        macartln2
        );
        println!("{} {}",
        macartln3, de
        );
    }
}

fn exit() {
    return;
}
