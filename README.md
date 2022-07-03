[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/v3.1.9/LICENSE-MIT)
[![Rust](https://github.com/LucasJaiser/mkDesktop/actions/workflows/rust.yml/badge.svg)](https://github.com/LucasJaiser/mkDesktop/actions/workflows/rust.yml)
[![creates.io](https://img.shields.io/badge/crates.io-1.1.0-orange)](https://crates.io/crates/mk_desktop)

# mkDesktop

mkDesktop is a CLI Application written in Rust which turns your input into a .desktop file. After you install a application from github or compile the code yourself, you often find the application not showing in your application finder of your Desktop Environment. For that you need a .desktop file in a specific location. To help you creating one this application comes in handy. It putts you through a guided Experience of making such a file. You dont have to have knowledge of where this file goes, what is the syntax of the file or what are valid values. The Application putts you all through this.  

## Usage
You can give the application the needed information through a guided mode or through CLI parameters. 

### Guided mode
Guided mode means you only can provide information to Application after it asks you for it. After running the application you will get asked some questions about the Application the .desktop file is for. This includes the name, Application type, which Categorie it falls under or which icon it should use to display in the menus. 
You can find more information about valid Types and Categories here: 

[Types](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html)

[Categories](https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry)

### Auto Detection
A third method of creating a .desktop file with this application is to use the --auto-detect flag. For this you have to be in the folder which includes the target application since the current folder you are in will be used to detect the name of the application. mkDesktop can only detect name and executable. For the other option it useses predefined values. 
Default values for Categorie is "Utility" and for AppType it is "Application".
You can change the default value by editing the config file.

### Templates
If you only want to have a template of a .desktop file you can use the --template flag. It outputs a .desktop file with no values in the current directory.  


### CLI usage (mk_desktop --help)

example: 'mk_desktop --name example --global global --icon example/resource/example.png -c Utility -a Application --exec chromium'

USAGE:

    mk_desktop [OPTIONS]

OPTIONS:

    -a, --app-type <APP_TYPE>
            Application Type (possible values: Application, Link, Directory)
            
            [default: Application]

    -A, --auto-detect
            Auto detect informations Only the following field can be detected: name (folder name),
            exec (file rights), icon (filename), global is predefined to "global", app-type is
            predefined to "Application"

    -c, --categories <CATEGORIES>
            Categories wich describes your Application, you can find possible Categories here:
            https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry
            
            [default: Utility]

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

    -p, --path <PATH>
            Path in which the file will be written. Warning: Overwrites the global and local Path(if
            the global parameter is set it will be no longer active. The app will use this Path)

    -t, --template
            Only Print out a template of the .desktop file

    -V, --version
            Print version information


## Configuration

The Config file defines some default Values which the app will use if you dont specify them.
Valid config are: 


global_path -> defines the path which will be used to globally install the .desktop file.

local_path -> defines the path which will be used to locally install the .desktop file.

categorie_default -> defines the default Categorie if not specied otherwise.

app_type_default -> defines the default AppType if not specified otherwise.


## Installation
### Requirements 
- cargo 
- rust 


For now you have to compile the application your self. This can be done through 

    git clone https://github.com/LucasJaiser/mkDesktop.git
    cd mkDekstop
    cargo install --path .

or you can use crates.io: 

    cargo install mk_desktop


## Contribution
You can find more information about how to Contribute on this file.

At the moment the application is just a little script, but there are lot more features planed right now. 
If you want more information on what features are comeing or you want to contribute check out the issue section. 
You are more then welcome to Contribute. 
 
