# Linking holochain entries
Solution to the [Getting Elements](https://holochain-gym.github.io/developers/basic/elements/) exercises on the Holochain Gym. 

`register_snacking` creates an entry and returns a struct containing the Header hash and the Entry Hash.  
`get_by_header_hash` returns the entry using the header hash as input.  
`get_by_entry_hash` likewise returns the entry, this time using the entry hash as input.  
`get_all_headers_from_content returns all headers associated with an entry hash. This function calculates the entry hash, then uses this to search the source chain. Empty search results return an empty vector.  

Comments in zomes/exercise/src/lib.rs are my notes from figuring this out. I had some help from the solutions, but made considerable strides without. 

### Setup - nix-shell
IMPORTANT: These need to be run in the correct nix-shell. 

In the base folder of this repository, developer-exercises, you will find
a `default.nix` file. Run the following command in your terminal:

```bash
nix-shell
```

The very first time you run this, it will take long time, somewhere between 20 and 80 minutes.
This is because it will download, install and compile everything you need. After that it will only take a second or two to run.

### Tests

```bash
cd tests
npm install
npm tests
```
