# Wir VS Virus
My repository for the German WirVsVirus hackathon.

## Backend (PANdemic SIMulation)

The recommended backend was written in Rust.

You can find it in `src/pansim`. To run it, do the following:
```
$ cd src/pansim
$ cargo run --release Config.toml
```

Or alternatively, you can run
```
$ cd src/pansim
$ cargo build --release
```

and then use 
```
target/pansim Config.toml
```
to run it so you don't have the compiler overhead.

You can change the settings in `src/pansim/Config.toml` to your liking.

If `save_to_file` is set to `true`, the program will also create an output file in `pansim_out`. The file is named with a time stamp, so the newest should be the last, alphabetically.

There are still many things to implement:
- cross-city transfers
- behaviour changes
- quarantine
- health service
- apparent statistics

## Frontend (PanVis)

The frontend was written in Javascript using Electron.

You can find it in `src/panvis`. To run it, do the following:
```
$ npm install
$ npm start
```
The file visualized can be found in `src/panvis/data.csv`. It can be exchanged to visualize differend developments of the virus.