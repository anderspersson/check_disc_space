# check_disc_space
A Disc Space Checker written in Rust.

> This software is "as-is" you can copy and use it, but i will not be responsible for it use,
> and if you missing feature, just clone the project and expand as you like, i will not
> build and expand it on demands. 

> **I Don't handling Pull Request. You are free to clone the projects as you like.**

This software is written in Rust, could probely be
written in a better way, but this project was for
me to learn basic of Rust.

## How to build
To build this you need Rust and a C++ compiler installed \
Read how on https://www.rust-lang.org/tools/install

## To test run the project:
cargo run

## To build the project
cargo build

### Result file is under
\target\debug\check_disc_space.exe (.exe if windows)

## To build a release version
cargo build -r

### Result file is under
\target\release\check_disc_space.exe (.exe if windows)

## Usage:
check_disc_space < Sizetype B|K|M|G > < drive/path > < verbose YES >

### Sizetype
B Bytesize \
K KB (KiloByte) \
M MB (MegaByte) \
G GB (GigaByte) \

### drive/path
In windows Drive name can be used like C: \
In Linux Path like \ can be used. \
Folders can be checked like \
c:\user  <Windows> \
\user    <Linux>c \

### verbose
YES you get a string back line C:\ have 12 GB \
NO och blank just return the number, witch is great if 
you plan to intergrat this in a script.





