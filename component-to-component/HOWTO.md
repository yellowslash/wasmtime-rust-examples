# How to get this going?

Have all the tools required installed, check project root README.md
Build with the build script included
Run with 
$wasmtime run --invoke 'main()' plugged.wasm

# Notes

See that component b's package is the same as packages a's. This is because the components need to be in the same package such that plugging works. At least this is my best undertanding of how it workds.