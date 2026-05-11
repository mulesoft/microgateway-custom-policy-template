# Microgateway Custom Policy template

This project is designed to scaffold custom policy implementation projects using [cargo generate](https://cargo-generate.github.io/cargo-generate/).

In short, this is the archetype of Flex Gateway custom policy implementations.

## FIPS Compliance

Projects can be created with FIPS compliance enabled using the `--fips` flag. This configures the PDK to use FIPS-compliant cryptographic modules. Note that this affects PDK cryptographic operations; custom policy code must also avoid non-compliant dependencies to maintain full compliance.

