# uwcwatcher-rs
Tool that limits CPU usage of Ubisoft Connect WebCore processes

It works by monitoring the CPU usage of each WebCore process, and if it goes over the user-defined threshold - it will be suspended. There is an option to pause the program (toggled by F2) which will cause the processes to be resumed. This is useful if you need to open the overlay (which requires the processes to be resumed).

## Installation

You can either clone this and build it yourself or use the supplied binaries for Windows

```bash
git clone https://github.com/weilbyte/uwcwatcher-rs
cd uwcwatcher-rs
cargo build
```

## Usage

Run the executable from either explorer or the terminal. You will be prompted to enter a threshold value; play around with this to find the perfect value for your CPU.

## Contributing
Pull requests are welcome. 
## License
[MIT](https://choosealicense.com/licenses/mit/)
