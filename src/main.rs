use colored::{ColoredString, Colorize};
fn main() {
    let mut os: &'static str = "mac"; // Set a OS
    let subsys: &'static str = ""; // Only for linux
    let mut de: &'static str = ""; // A Desktop Enviroment

    let mut _arch = false;
    let mut _ubuntu: bool = false;
    let mut _mint: bool = false;
    let mut _linux: bool = false;

    // ART
    let mut _mac: bool = false;
    let mut _linux: bool = false;

    let macartln1: ColoredString = "/   \\".red();
    let macartln2: ColoredString = "| M |".yellow();
    let macartln3: ColoredString = "\\___/".green();

    let linuxln1: ColoredString = "/   \\".truecolor(128, 128, 128);
    let linuxln2: ColoredString = "\\___/".truecolor(128, 128, 128);
    let linuxln3: ColoredString = "/ L \\".truecolor(128, 128, 128);

    let archln1: ColoredString = "  ,   ".cyan();
    let archln2: ColoredString = " / \\ ".cyan();
    let archln3: ColoredString = "/   \\".cyan();

    let ubuntuln1: ColoredString = "o__, ".truecolor(221, 72, 20);
    let ubuntuln2: ColoredString = "    O".truecolor(221, 72, 20);
    let ubuntuln3: ColoredString = "0--' ".truecolor(221, 72, 20);

    let mintln1: ColoredString = "/ M ,".green();
    let mintln2: ColoredString = "\\___/".green();
    let mintln3: ColoredString = "/".green();

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
