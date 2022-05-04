# Sysinfo-gui

A lightweight cross-platform system-monitoring [fltk](https://github.com/fltk-rs/fltk-rs) gui application based on [sysinfo](https://github.com/GuillaumeGomez/sysinfo).

The UI design is inspired by [stacer](https://github.com/oguzhaninan/Stacer).

The svg icons are taken from: https://icons.getbootstrap.com/. The ascending and descending icons from https://www.svgrepo.com/.

The font is Roboto Medium.

## Screenshots

- Dashboard
![image](https://user-images.githubusercontent.com/37966791/166832794-6d56d40a-f07d-446f-833b-e60fba3c0c6f.png)

- Processes
![image](https://user-images.githubusercontent.com/37966791/166832866-0b533faf-6833-4b3d-831c-20c21bee2487.png)

- Cpu
![image](https://user-images.githubusercontent.com/37966791/166832912-5dabfa08-2491-4bd8-bc15-ffa3a49ba11f.png)

- Memory
![image](https://user-images.githubusercontent.com/37966791/166832991-65bd3782-b5ed-40f7-800c-b0d3f3a5e611.png)

- Disk info
![image](https://user-images.githubusercontent.com/37966791/166833062-154428d8-7ecf-4b3f-9875-7397f84c7b32.png)

- Networking
![image](https://user-images.githubusercontent.com/37966791/166833123-484adeac-0b23-4b0a-bfb0-767c1b4856cd.png)

- Settings
![image](https://user-images.githubusercontent.com/37966791/166833165-6e206910-7314-4d23-9734-c1b20774f6ca.png)

## Features
- Supports window transparency.
- Realtime monitoring.
- End processes by sending a kill signal in the processes view.
- Both dark and light modes.
![image](https://user-images.githubusercontent.com/37966791/166833290-5161965a-f730-4f76-9845-0e0d38db6f96.png)

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
