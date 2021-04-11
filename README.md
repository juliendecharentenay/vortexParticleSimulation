# VortexParticleSimulation
This repository contains a library and command line interface to perform fluid dynamics simulation based on the vortex particle method.
The methodology employed is inspired from the Fluid Simulation for Video Games series by Dr M J Gourlay - see [www.mijagourlay.com](https://www.mijagourlay.com/fluid).

# Installation and Usage

Prerequisites: [rust](https://www.rust-lang.org/tools/install)

## Debug mode
Compile using:

```
vortexParticleSimulation> cargo build
```

Run - displaying the help:
```
vortexParticleSimulation> .\target\debug\cli.exe --help
```

## Release mode:
Compile using:
```
vortexParticleSimulation> cargo build --release
```

Run - displaying the help:
```
vortexParticleSimulation> .\target\release\cli.exe --help
```

## Others:
* Run the unit testing:
```
vortexParticleSimulation> cargo test
```

* Run an example test case after compilation in release mode:
```
vortexParticleSimulation> .\target\release\cli.exe --????
```



# Other
An online version of this fluid simulation tool will be/is available at [cfd-webassembly.com](https://www.cfd-webassembly.com).

I am posting updates on this project and other projects at [charentenay.me](https://www.charentenay.me/) and on [medium.com](https://julien-decharentenay.medium.com/).
