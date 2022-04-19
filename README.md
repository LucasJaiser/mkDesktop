[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/clap-rs/clap/blob/v3.1.9/LICENSE-MIT)

# mkDesktop
## Description
mkDesktop is a CLI Application written is Rust which turns your input into a .desktop file. After you install a application from github or compile the code yourself, you often find the application not showing in your application finder of your Desktop Environment. For that you need a .desktop file in a specific location. To help you creating one this application comes in handy. It putts you through a guided Experience of making such a file. You dont have to have knoledge of where this file goes, what is the syntax of the file or what are valid values. The Application putts you all through this.  

## Usage
In this Version we only have the guided form of input. This means you only can provide information to Application after it asks you for it. After running the application you will get asked some questions about the Application the .desktop file is for. This includes the name, Application type, Which Categorie it falls under or which icon it should use to display in the menus. 
You can find more information about valid Types and Categories here: 
[Types](https://specifications.freedesktop.org/desktop-entry-spec/latest/ar01s06.html)
[Categories](https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html#category-registry)

## Installation
### Requirements 
- cargo 
- rust 

For now you have to compile the application your self. This can be done through `cargo build`.

## Contribution
You can find more information about how to Contribute on this file.

At the moment the application is just a little script, but there are lot more features planed right now. 
If you want more information on what features are comeing or you want to contribute check out the issue section. 
You are more then welcome to Contribute. 
 
