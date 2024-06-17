# Set license header in files

## Install licensure

You need install the licensure tool:

```sh
cargo install licensure
```

## Run licensure to add / update headers

Run from root of repository: 

```sh
find ./ -name "*.rs" -exec ~/.cargo/bin/licensure --in-place --verbose {} \;
```
