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

Share a link that expires in 4 weeks:

`./sialo share -s <OBJECT_HASH> -t 4w`

Share with an exact expiration date:

`./sialo share -s <OBJECT_HASH> -t 2026-12-31T23:59:59Z`

Supported duration suffixes: `h` (hours), `d` (days), `w` (weeks). Exact ISO 8601 timestamps are also accepted.

Use `sialo share -h` for a quick summary or `sialo share --help` for full details with examples.

## Embedding Sia Content in HTML

HTML pages hosted on Sia can reference other Sia-hosted content using the `sia://` protocol. The Sia Browser intercepts these URLs and downloads + decrypts the content directly from storage hosts via WebTransport.

To create a `sia://` URL, take any share URL and replace the `https://` prefix with `sia://`:

```
Share URL:  https://app.sia.storage/objects/<hash>/shared?sv=...&ss=...#encryption_key=...
Embed URL:  sia://app.sia.storage/objects/<hash>/shared?sv=...&ss=...#encryption_key=...
```

### Images

```html
<img src="sia://app.sia.storage/objects/<hash>/shared?sv=...&ss=...#encryption_key=..." alt="My Image" />
```

### Videos

```html
<video src="sia://app.sia.storage/objects/<hash>/shared?sv=...&ss=...#encryption_key=..." preload="none"></video>
```

Use `preload="none"` so videos don't start downloading until the user plays them.

### Links to Other Sia Pages

```html
<a href="sia://app.sia.storage/objects/<hash>/shared?sv=...&ss=...#encryption_key=...">
  Read more
</a>
```

Clicking a `sia://` link opens the content in a new Sia Browser tab. This works for HTML pages, PDFs, images, or any other file type.

### Workflow

1. Upload the assets (images, videos, etc.) with `sialo upload`
2. Generate share URLs with `sialo share -t <duration>`
3. Write your HTML page using `sia://` URLs for embedded content
4. Upload the HTML page itself with `sialo upload`
5. Share the HTML page with `sialo share -t <duration>`

See `flowers.html` in this repo for a complete example of an HTML page with 10 embedded `sia://` images.

### Misc

RUST_LOG env variable supports filtering out noisy log messages:

`RUST_LOG="debug,quinn::connection=warn" APP_KEY="4a5db12851738b74a7f6ad8a2092d23980e455a03d99f373b69e9629eccf2549b1c33f4e3c999e5d3f94b669932f96f7b5cf4ffe55ad69f2202ab38845a250fa" ./sialo upload --file hello_world.txt`