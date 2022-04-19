[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/v3.1.9/LICENSE-MIT)
[![Rust](https://github.com/LucasJaiser/mkDesktop/actions/workflows/rust.yml/badge.svg)](https://github.com/LucasJaiser/mkDesktop/actions/workflows/rust.yml)


# mkDesktop

mkDesktop is a CLI Application written is Rust which turns your input into a .desktop file. After you install a application from github or compile the code yourself, you often find the application not showing in your application finder of your Desktop Environment. For that you need a .desktop file in a specific location. To help you creating one this application comes in handy. It putts you through a guided Experience of making such a file. You dont have to have knoledge of where this file goes, what is the syntax of the file or what are valid values. The Application putts you all through this.  

## Usage
You can give the application the needed information through a guided mode or through CLI parameters. Guided mode means you only can provide information to Application after it asks you for it. After running the application you will get asked some questions about the Application the .desktop file is for. This includes the name, Application type, which Categorie it falls under or which icon it should use to display in the menus. 

You can find more information about valid Types and Categories here: 

[Types](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html)

[Categories](https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry)

### CLI usage (mk_desktop --help)
USAGE:
    mk_desktop [OPTIONS] --name <NAME> --app-type <APP_TYPE> --exec <EXEC> --icon <ICON>

OPTIONS:
    -a, --app-type <APP_TYPE>
            Application Type (possible values: Application, Link, Directory)

    -c, --categories <CATEGORIES>
            Categories wich describes your Application, you can find possible Categories here:
            https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
            
            [default: ]

    -e, --exec <EXEC>
            The binary or .sh etc. which should be executed

    -g, --global <GLOBAL>
            Should mkDesktop install the in global Directory or in the Local only for the current
            user
            
            [default: global]

    -G, --guided
            Starts the guided mode of mkDesktop, you will get asked step by step all needed
            Information. Good for beginners

    -h, --help
            Print help information

    -i, --icon <ICON>
            The Icon wich will be displayed with this Application

    -n, --name <NAME>
            Name of the File you want to create

    -V, --version
            Print version information

## Installation
### Requirements 
- cargo 
- rust 

For now you have to compile the application your self. This can be done through `cargo install --path .`.

## Contribution
You can find more information about how to Contribute on this file.

At the moment the application is just a little script, but there are lot more features planed right now. 
If you want more information on what features are comeing or you want to contribute check out the issue section. 
You are more then welcome to Contribute. 
 
