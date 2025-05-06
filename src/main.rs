use colored::{ColoredString, Colorize};
pub fn main() {
    let mut os = "linux"; // Set a OS
    let subsys = "ubuntu"; // Only for linux
    let mut de = "KDE Plasma 9"; // A Desktop Enviroment

    let mut _arch = false;
    let mut _ubuntu: bool = false;
    let mut _mint: bool = false;
    let mut _linux: bool = false;

    // ART
    let mut _mac = false;
    let mut _linux = false;

    let macartln1 = "/   \\".red();
    let macartln2 = "| M |".yellow();
    let macartln3: ColoredString = "\\___/".green();

    let linuxln1 = "/   \\".truecolor(128, 128, 128);
    let linuxln2 = "\\___/".truecolor(128, 128, 128);
    let linuxln3 = "/ L \\".truecolor(128, 128, 128);

    let archln1 = "  ,   ".cyan();
    let archln2 = " / \\ ".cyan();
    let archln3 = "/   \\".cyan();

    let ubuntuln1 = "o__, ".truecolor(221, 72, 20);
    let ubuntuln2 = "    O".truecolor(221, 72, 20);
    let ubuntuln3 = "0--' ".truecolor(221, 72, 20);

    let mintln1 = "/ M ,".green();
    let mintln2 = "\\___/".green();
    let mintln3 = "/".green();

    if os == "" {
        exit();
    }

    if os == "linux" {
        _linux = true;
        if subsys == "arch" {_arch = true; os = "Arch Linux";}
        if subsys == "ubuntu" {_ubuntu = true; os = "Ubuntu Linux"}
        if subsys == "mint" {_mint = true; os = "Linux Mint"}
        if subsys == "" {_linux = true; os = "Linux"}
    }

    if os == "mac" {
        _mac = true;
        os = "MacOS";
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
            println!("{}",
            archln2
            );
            println!("{} {}",
            archln3, de
            );
        } else if _ubuntu == true {
            println!("{} {}",
            ubuntuln1, os
            );
            println!("{}",
            ubuntuln2
            );
            println!("{} {}",
            ubuntuln3, de
            );
        } else if _mint == true {
            println!("{} {}",
            mintln1, os
            );
            println!("{}",
            mintln2
            );
            println!("{} {}",
            mintln3, de
            );
        } else if _linux == true {
            println!("{} {}",
            linuxln1, os
            );
            println!("{}",
            linuxln2
            );
            println!("{} {}",
            linuxln3, de
            );
        }
    }
}

fn exit() {
    return;
}
