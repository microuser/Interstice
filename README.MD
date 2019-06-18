# Interstice
It literally means:
`an intervening space, especially a very small one`


# What is this?
This is a coding project. In fact is a my first rust program. I love writing reproducable documentation so this is my step by step through process as I made this example program. The primise is that a JSON file contains some date ranges, and I want to find the Interstice of days which can fit reservations in them without creating unusable days. For example if I have a week (where all reservations are for the night of, like a hotel), day 1 though 6. it would be inadvisable to allow a reservation from day 3 though 5, because few reservations in my proverbial puzzle would be for just 2 days from day 1 to day 2, while also preventing the same issue at the other size for day 6 and 7. There is a specific keywork i am avoidding on purpose, perhaps it has the same name as a clothing company. This is because this code will be published on github and I desire an audience of those wanting to learn Rust language, rather than people using gogole to cheat to get free code.

# Why Rust?
Beacuse I'm learning something new. This example is childsplay, but has some nice elements to learn how to do things I do in other languages. I want to focus on Efficient, extendable, and testable code.

# What is different about rust
This language shares many things with other languages, but yet there is something which will affect the structure more than others:

- Ownership of vairables if quite interesting. Essentially, read only pointers if allow a shadow copy. Or literally returing multiple objects as a tuple if you want to keep ownership of an object.
- No more null patterns. Seriouslly, no more null objects, just a new way around it with a compiler check. But combined witht he new structure of a switch case using match, we now have code which won't have null exceptions.


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
I changed the value for the key "program" to : "program": "${workspaceFolder}/target/debug/interstice.exe",

### Trying some debugging
research better ways to build: https://fungos.github.io/blog/2017/08/12/setting-up-a-rust-environment-on-windows/

Set some breakpoints in main.rs

Type in new terminal inside VScode
`cargo build`

Use Debug Menu->  Start Debugging (F5)


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

### Learning Rust tests on Structs
skim through examples. 
https://doc.rust-lang.org/book/ch11-03-test-organization.html

# Lets Commit
Documentation: https://help.github.com/en/articles/adding-an-existing-project-to-github-using-the-command-line

Since `cargo new Interstice` was used, we have a local github repository. But I want it in github. Use your github account and create an empty repository with no Readme file or license.

Open up the git prompt and run:
`ssh-keygen.exe`
notice that it created some files and stored them in /c/Users/username/.ssh/

Copy that pub key to github.com ssh keys section
`cat /c/Users/micro/.ssh/id_rsa.pub`

Since we are typing this up. Lets make sure we got out project added to our local repository
`git add *`

`git commit -m 'writing git documentation'`

Copy the  ssh command from your repo :git@github.com:microuser/Interstice.git

declare the local repo has a remote origin
`git remote add origin git@github.com:microuser/Interstice.git`
verify url
`git remote -v`
Notice that we have to lines
now lets sync them. We will push to master branch since that is what we have been using.
`git push origin master`

since we have been typing in this document, lets verify changes propigate. Save the MD file i am typing right now.
`git status`

notice we have a modified README.MD. Commit all unstaged changes and give it a message
`git commit -a -m "checking chained connection of repos"`
but when we attempt `git push` we notice that we still need to set upstream master. So lets do that.

`git push --set-upstream origin master`

Now check you files on your github.

And lets change our branch to a dev branch to keep off master for now.

remember: in the future it will default to master, to get a branch in specific use the b option `git checkout branchname`. Even creating a branch requires switching to it. so lets do both.

create branch
`git branch dev`
`git checkout dev`

lets verify this works.
`git commit -a "just created a dev branch"




# Lets review our objective
## Accoring to requirements:
1. Load the JSON file
2. Filter by date ranges
    * date range criteria
    * apply interstice technology
      * define what does this mean?
3. Output the "cabin name"s to terminal

## Checklists
1. Does it work? 
   1. Expected output is `
"Comfy Cabin"
"Rickety Cabin"
"Cabin in the Woods"
`   
   2. Tests for Search strut
   3. Tests campsites strut
   4. Tests for Reservations strut
   5. Tests for Parsing DateTime from strings
   6. Tests & code for finding Daterange unions
   7. Tests & code for finding Daterange partial overlaps 
   8. Tests & code for finding Daterange no overlap
   9. Test consistancy check
      1.  are they maintainable? 
      2.  are they well structured
   10. Code consistancy check
       1.  Can this be extenable?
       2.  Can you use this as an interface for something else rather than just CLI?
           1.  Consider a docker microservice with share to xfer file
   11. Integration Test

## Lets Code the JSON loader to object with a simple test


