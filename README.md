# Sysinfo-gui

A lightweight cross-platform system-monitoring [fltk](https://github.com/fltk-rs/fltk-rs) gui application based on [sysinfo](https://github.com/GuillaumeGomez/sysinfo).

The UI design is inspired by [stacer](https://github.com/oguzhaninan/Stacer).

The svg icons are taken from: https://icons.getbootstrap.com/.
The font is Roboto Medium.

## Screenshots

- Dashboard
![image](https://user-images.githubusercontent.com/37966791/166341488-1ccf1a98-d490-4988-a200-ab7a7cab3968.png)

- Processes
![image](https://user-images.githubusercontent.com/37966791/166341586-be45393e-0f22-4b40-9995-ebd4b28f18b2.png)

- Cpu
![image](https://user-images.githubusercontent.com/37966791/166341628-37271b17-481f-46b8-87fd-6a34c3ec296c.png)

- Memory
![image](https://user-images.githubusercontent.com/37966791/166341679-a2a81bb5-f3e3-43a3-ab1f-95f683066946.png)

- Disk info
![image](https://user-images.githubusercontent.com/37966791/166341734-a4f73c8f-776f-418c-be7d-459219703c48.png)

- Networking
![image](https://user-images.githubusercontent.com/37966791/166341774-73a47feb-136f-4a7a-8e17-8973525e7ae1.png)

- Settings
![image](https://user-images.githubusercontent.com/37966791/166341827-cbbedc72-7138-4b63-aaa1-e07d0d103bdc.png)

## Features
- Both dark and light modes.
- Supports window transparency.
- Realtime monitoring.
- End processes by sending a kill signal in the processes view.

## Prebuilt standalone releases:

These are built using github actions, and can be found here:
https://github.com/MoAlyousef/sysinfo-gui/releases

## TODO
- The codebase needs some refactoring, maybe the View struct can hold data instead of using lazy_statics.
- Support more things in the right click popup menu in the processes view.
- Add downloand and upload speed.
- Add more setting tweaks.
