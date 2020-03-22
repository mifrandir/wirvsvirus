# Wir VS Virus
My repository for the German WirVsVirus hackathon.

To install this, run
```
$ git clone https://github.com/miltfra/wirvsvirus.git
```

To follow the rest of the instructions `cd` into the directory:
```
$ cd wirvsvirus
```

## Strcutre

This project include two parts.

1. PANSIM - A simulation that provides data on a theoretical development of a pandemic.
2. PANVIS - A visualizer for the data on the development of a pandamic.


## Backend (PANdemic SIMulation)

Author: Franz Miltz

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

### Features

- efficient simulation of more than 10 million people
- full customizability through `Config.toml`
- social structure through groups (households, districts, cities) levels of connectedness
- random relations between people in households
- health system with capacity per city
- lethality based on treatment
- spread based on quarantine

### Caveats

- based on no scientificly researched model
- simulation of every single person is unfeasible, therefore some abstractions are being made
- data for lethality, effectivity of treatment and mobility between social groups is unknown and undeterminable

### TODO

- cross-city transfers
- behaviour changes
- quarantine
- health service
- apparent statistics

## Frontend (PANdemic VISualization)

Author: Richard Schiemenz

The frontend was written in Javascript using Electron.

You can find it in `src/panvis`. To run it, do the following:
```
$ cd src/panvis
$ npm install
$ npm start
```
The file visualized can be found in `src/panvis/data.csv`. It can be exchanged to visualize differend developments of the virus.
