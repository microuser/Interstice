###
Setting Up dev environment
##In Windows

install git if you don't have it. I chose VScode as text editor
I opened a terminal and connected it to me.
`git config --global user.name "MY NAME "`
`git config --global user.email "MY EMAIL"`


Get Visual Studio Code from ninite.com
Use VSCode plugin area to install C/C++ : https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools
Use VSCode plugin area to install Native Debug : https://marketplace.visualstudio.com/items?itemName=webfreak.debug

Use VSCode plugin area to install Better TOML

Close and reopen VS code. It prompted me to install RLS. if not, install that plugin as well.

Use VSCode plugin area to install crates to read TOML files

Download and install the Build Tools from mirosoft : https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 with the package https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16

Download and install Rust from https://rustup.rs/


Notice that when you install it changes your path, which indicates you will need to close your consoles and reopen them. But if you don't you can of course just call cargo directly  C:\Users\micro\.cargo\bin

cd into the directory of choice ~/code
Create the project using: `cargo new interstice`

Use visual studio code to Add folder to workspace

edit Cargo.toml

edit ./interstice/src/main.rs, and add some random lines with code. 
Place breakpoints. 

Note: I followed this tutorial: https://www.forrestthewoods.com/blog/how-to-debug-rust-with-visual-studio-code/

in settings I allowed Breakpoints everywhere in any file. 

Use Menu: Debug -> Start Debugging
Since I'm on windows I choose C++ Windows
Notice that a launch.json appeared. You can delete this if you mess it up.
I changed the value for the key "program" to : "${workspaceFolder}/interstice.exe",




