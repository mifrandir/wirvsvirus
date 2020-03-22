# Wir VS Virus
My repository for the German WirVsVirus hackathon.

## Backend (PANdemic SIMulation)

The recommended backend was written in Rust.

You can find it in `src/pansim`. To run it, do the following:
```
$ cd src/pansim
$ cargo run Config.toml
```

You can change the settings in `src/pansim/Config.toml` to your liking.

You can find the python backend in `src/simulation`.

How do I use it? For now, just run

```
$ python src/pansim-py/pansim
```

You can find some example outputs in the `data` directory.

There are still many things to implement:
- cross-city transfers
- behaviour changes
- quarantine
- health service
- apparent statistics
