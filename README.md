# Sysinfo-gui

![alt_test](https://github.com/MoAlyousef/sysinfo-gui/raw/main/assets/icon.png)

A lightweight cross-platform system-monitoring [fltk](https://github.com/fltk-rs/fltk-rs) gui application based on [sysinfo](https://github.com/GuillaumeGomez/sysinfo).

The UI design is inspired by [stacer](https://github.com/oguzhaninan/Stacer).

The svg icons are taken from: https://icons.getbootstrap.com/. The ascending and descending icons from https://www.svgrepo.com/.

The font is Roboto Medium.

## Screenshots

- Dashboard

![image](https://user-images.githubusercontent.com/37966791/209539808-dba1412c-a38f-4fdd-b143-9d991d1e4e49.png)

- Processes

![image](https://user-images.githubusercontent.com/37966791/209539668-3dcc559b-d9ad-47f3-9055-c2216e29fac5.png)

- Cpu

![image](https://user-images.githubusercontent.com/37966791/209539860-8d28adf5-13de-4a0b-8600-fbeed6be95bc.png)

- Memory

![image](https://user-images.githubusercontent.com/37966791/209539910-845c082b-6e0b-4467-b00e-167fe15fa010.png)

- Disk info

![image](https://user-images.githubusercontent.com/37966791/209539970-521037b9-6fd1-495f-a92f-c42daeb56d70.png)

- Networking

![image](https://user-images.githubusercontent.com/37966791/209540014-1421639c-6430-490a-9c75-74c40228717f.png)

- Settings

![image](https://user-images.githubusercontent.com/37966791/209540135-5a390e60-849c-4b93-b05a-a411924642e2.png)

## Features
- Supports window transparency.
- Realtime monitoring.
- End processes by sending a kill signal in the processes view.
- Both dark and light modes.

![image](https://user-images.githubusercontent.com/37966791/209540190-4fd60269-34ad-4775-85f2-3b64be5763f1.png)

## Getting the application:

- Prebuilt standalone releases:

These are built using github actions, and can be found here:
https://github.com/MoAlyousef/sysinfo-gui/releases

- You can use cargo to install the application:
```bash
cargo install sysinfo-gui
```

## Building from source:

You can clone the repo and build using:
`cargo build --release`

If you would like to use the bundled version of fltk-rs (for supported platforms (x86_64 windows, macos and linux)):
`cargo build --features=fltk/fltk-bundled --release`

## TODO
- Support more things in the right click popup menu in the processes view.
- Map uid to User and display a user name in the processes view.
- Add downloand and upload speed.
- Use better colors for the cpus. 
- Add more setting tweaks.
