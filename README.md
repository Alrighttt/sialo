# Usage

See 
`./sialo help`

## Register

Register command allows authorizing an application. 

Use the default indexer url and provide seed as an argument:

`./sialo register --seed-phrase "audit audit audit audit audit audit audit audit audit audit audit audit"`

Set a custom indexer url and provide seed as an env varible:

`INDEXER_URL="http://127.0.0.1:9982" SEED_PHRASE="audit audit audit audit audit audit audit audit audit audit audit audit" ./sialo register`

Set a custom AppMetadata JSON file (see example_json.json in this repo):

`APP_METADATA="example_app.json" SEED_PHRASE="audit audit audit audit audit audit audit audit audit audit audit audit" ./sialo register`

## Upload

`./sialo upload --app-key 4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa hello_world.txt`

or 

`APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa" ./sialo upload hello_world.txt`

or 

```
export APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa"
./sialo upload hello_world.txt
```

## Download

`APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa" ./sialo download bd00710a8182b18e4c2263f69e3a5785b275e0911cc4bbadc555c743576772bd --output-file some.file`

## Delete

`APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa" ./sialo delete bd00710a8182b18e4c2263f69e3a5785b275e0911cc4bbadc555c743576772bd`

## PruneSlabs

`./sialo prune-slabs --app-key 4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa`

## Share

./sialo share --app-key 4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa --object-hash 11ac3cca32e44aaa436edac9fe49059a1215a108b5f2029d3f31783b9307b3a0 --share-until 1

### Misc

RUST_LOG env variable supports filtering out noisy log messages:

`RUST_LOG="debug,quinn::connection=warn" APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa" ./sialo upload --file hello_world.txt`