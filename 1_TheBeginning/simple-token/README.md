Helpful places to go when you are brand new:
https://docs.radixdlt.com/main/scrypto/introduction.html

Helpful places to go when you are looking for examples:
https://github.com/radixdlt/radixdlt-scrypto/tree/main/examples
https://github.com/cbisaillon/Scrypto-Documentation
https://github.com/radixdlt/community-scrypto

Basic flow to first deploy once on Radix Engine Simulator (resim) once you clone this is:
```
resim reset
resim new-account
export account=<account>
export pubkey=<pubkey>
resim publish .
export package=<package_address>
resim show $package
resim call-function $package NewToken <token_name> <token_symbol> <toke_supply>
export component=<component_address>
resim show $component
```