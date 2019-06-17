# Setting Up Dev environment
## In Windows

### Get git
install git if you don't have it. I chose VScode as text editor
I opened a terminal and connected it to me.
`git config --global user.name "MY NAME "`
`git config --global user.email "MY EMAIL"`

### Get Visual Studio Code and Configure

Get Visual Studio Code from ninite.com
Use VSCode plugin area to install C/C++ : https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools
Use VSCode plugin area to install Native Debug : https://marketplace.visualstudio.com/items?itemName=webfreak.debug

Use VSCode plugin area to install Better TOML

Close and reopen VS code. It prompted me to install RLS. if not, install that plugin as well.

Use VSCode plugin area to install crates to read TOML files

### Get Microsoft build tools for debugging and compiling

Download and install the Build Tools from mirosoft : https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019 with the package https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16 note: for me this got the file: vs_buildtools__2031235521.1560808878.exe and installed it, which it then downloaded and installed more actual files. I chose C++ build tools and some .NET desktop build tools cause they are usefull for C#. Whoa nelly, this was 7GB of junk.

### Get Rust itself

Download and install Rust from https://rustup.rs/

Notice that when you install it changes your path, which indicates you will need to close your consoles and reopen them. But if you don't you can of course just call cargo directly  C:\Users\micro\.cargo\bin

## Setup a new Project
### The skeleton

note if this does not work, then try with a new console instance.

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

### Adding JSON serialization
research the best cargo file is Serde JSON
Documentation: https://crates.io/crates/serde_json
Add this to Cargo.toml under dependencies
`[dependencies]`
`serde_json = "1.0"`

### Adding File Parsing
research file streams in Rust
Documentation: https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html
note the namespaces are already available:
`use std::fs::File;`
`use std::io::{Write, BufReader, BufRead, Error};`

### Combining JSON from File Knowledge
research BufReader and serde_json
Documentation: https://docs.serde.rs/serde_json/fn.from_reader.html

### 
