
pub fn main() {
    let mut os = "mac"; // Set a OS
    let subsys = ""; // Only for linux
    let mut de = ""; // A Desktop Enviroment
    let art = ""; // Overides Art

    let mut _arch = false;
    let mut _ubuntu: bool = false;
    let mut _mint: bool = false;
    let mut _linux: bool = false;

    // ART
    let mut _mac = false;
    let mut _linux = false;

    let macartln1 = "/   \\";
    let macartln2 = "| M |";
    let macartln3: &'static str = "\\___/";

    let linuxln1 = "/   \\";
    let linuxln2 = "\\___/";
    let linuxln3 = "/ L \\";

    let archln1 = "  / \\";
    let archln2 = " /  \\";
    let archln3 = "/   \\";

    if art != "" {
        if art == "mac" {_mac = true;}
        if art == "linux" {_linux = true;}
    }

    if os == "" {
        exit();
    }

    if os == "linux" {
        if subsys == "arch" {_arch = true; os = "Arch Linux";}
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
    if _linux == true {
        if _arch == true {
            println!("{} {}",
            archln1, os
            );
        }
    }
}

fn exit() {
    return;
}
