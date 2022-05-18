# Sysinfo-gui

A lightweight cross-platform system-monitoring [fltk](https://github.com/fltk-rs/fltk-rs) gui application based on [sysinfo](https://github.com/GuillaumeGomez/sysinfo).

The UI design is inspired by [stacer](https://github.com/oguzhaninan/Stacer).

The svg icons are taken from: https://icons.getbootstrap.com/. The ascending and descending icons from https://www.svgrepo.com/.

The font is Roboto Medium.

## Screenshots

- Dashboard
![image](https://user-images.githubusercontent.com/37966791/169169613-af34be21-5b7d-4176-9e45-dfe763482996.png)

- Processes
![image](https://user-images.githubusercontent.com/37966791/169169714-24a5f233-e391-44c9-9c39-ebd42a5ab429.png)

- Cpu
![image](https://user-images.githubusercontent.com/37966791/169169787-668c23e5-7133-4e32-bc39-9c687a903150.png)

- Memory
![image](https://user-images.githubusercontent.com/37966791/169169841-bd6af58f-785b-4429-9b86-c1c5e160f9ab.png)

- Disk info
![image](https://user-images.githubusercontent.com/37966791/169169890-8fcb9d00-6f55-45d4-926c-781fb8066e4c.png)

- Networking
![image](https://user-images.githubusercontent.com/37966791/166833123-484adeac-0b23-4b0a-bfb0-767c1b4856cd.png)

- Settings
![image](https://user-images.githubusercontent.com/37966791/166833165-6e206910-7314-4d23-9734-c1b20774f6ca.png)

## Features
- Supports window transparency.
- Realtime monitoring.
- End processes by sending a kill signal in the processes view.
- Both dark and light modes.
![image](https://user-images.githubusercontent.com/37966791/169170165-f0417ec7-a1dd-4282-a9b1-7bffdc3fc3bc.png)

## Getting the application:

- Prebuilt standalone releases:

These are built using github actions, and can be found here:
https://github.com/MoAlyousef/sysinfo-gui/releases

- You can use cargo to install the application:
`cargo install sysinfo-gui`

## Building from source:

You can clone the repo and build using:
`cargo build --release`

If you would like to use the bundled version of fltk-rs (for supported platforms (x86_64 windows, macos and linux)):
`cargo build --features=fltk/fltk-bundled --release`

## TODO
- The codebase needs some refactoring.
- Support more things in the right click popup menu in the processes view.
- Add downloand and upload speed.
- Use better colors for the cpus. 
- Add more setting tweaks.
