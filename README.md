# VortexParticleSimulation
VortexParticleSimulation project implement a fluid dynamics simulation engine based on the vortex particle method.
The methodology employed is inspired from the Fluid Simulation for Video Games series by Dr M J Gourlay - see [www.mijagourlay.com](https://www.mijagourlay.com/fluid).

This repository contains the following: 
- `www`: a VueJS based website that is used for the front-end/gui to the CLI and the webassembly port;
- `rust`: a Rust implementation of the vortex particle simulation engine. The directory is itself split into the following sub-directories:
  - `rust/cli`: implementation of command line interface to the simulation engine;
  - `rust/vortex-particle-simulation`: implementation of the simulation engine itself;
  - `rust/wasm`: implementation of the bindings to webassembly
- `examples`: example of input data for the command line interface.

# Installation and Usage

Prerequisites: 
* [rust](https://www.rust-lang.org/tools/install) - for the rust implementation of the command line interface and engine;
* [Node.js](https://nodejs.org/en/) - for the compilation of the VueJS website.

[To check]: webassembly

## Website
Use as follows:

1. Install pre-requisites:

```
vortexParticleSimulation>cd www
vortexParticleSimulation\www>npm install
```

2. Run local webserver:
```
vortexParticleSimulation\www>npm run serve
```

3. Compile webpages to `dist` directory:
```
vortexParticleSimulation\www>npm run build
```

Note: the above compile the rust solver to webassembly and
package it to the website using the [WasmPackPlugin](https://github.com/wasm-tool/wasm-pack-plugin)
with no further user action required.

## Rust command line interface (CLI)
### Debug mode
Compile using:

```
vortexParticleSimulation\rust> cargo build
```

Run - displaying the help:
```
vortexParticleSimulation\rust> .\target\debug\cli.exe --help
```

### Release mode:
Compile using:
```
vortexParticleSimulation\rust> cargo build --release
```

Run - displaying the help:
```
vortexParticleSimulation\rust> .\target\release\cli.exe --help
```

### Others:
* Run unit testing:
```
vortexParticleSimulation\rust> cargo test
```

* Run an example test case after compilation in release mode:
```
vortexParticleSimulation\rust> .\target\release\cli.exe --init ..\examples\vortex-ring.json --run --save vortex-ring.sim --csv Out
```

* Restart from an existing case and overwrite:
```
vortexParticleSimulation\rust> .\target\release\cli.exe --restart vortex-ring.sim --run --save vortex-ring.sim --csv Out
```

Using the option `--csv Out` in the two examples above will output a number of CSV files - one every time-step - that contains the 
vortex particle location and vorticity magnitude. This CSV file can be loaded in ParaView and then plotted using the <i>Table to Point</i> 
filter.

# Other
An online version of this fluid simulation tool will be/is available at [cfd-webassembly.com](https://www.cfd-webassembly.com).

I am posting updates on this project and other projects at [charentenay.me](https://www.charentenay.me/) and on [medium.com](https://julien-decharentenay.medium.com/).
