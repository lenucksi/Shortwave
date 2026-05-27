# Discovery Scripts

Shortwave lets you write station discovery scripts in [Rhai](https://rhai.rs/), a
safe, embeddable scripting language for Rust. Scripts fetch radio station data
from web APIs or scrape HTML pages, then return structured results that
Shortwave can import.

They run inside a sandboxed Rhai engine with configurable resource limits and
access to a small set of purpose-built functions.

---

## Quick Start

Create a file called `hello-world.rhai`:

```rhai
json_stringify(#{
    "provider": "Hello World",
    "stations": [
        #{
            "name": "My Test Station",
            "stream_url": "https://example.com/stream.mp3",
            "stream_urls": [
                #{ "url": "https://example.com/stream.mp3", "codec": "MP3", "bitrate": 128, "tls": false }
            ],
            "tags": "test,demo",
            "country": "US"
        }
    ]
})
```

Run it:

```bash
cargo run --bin shortwave-script -- run hello-world.rhai
```

---

## Rhai Crash Course

Rhai syntax is similar to JavaScript/Rust. You only need a small subset to
write discovery scripts.

### Variables

```rhai
let name = "SomaFM";       // string
let count = 42;            // integer
let enabled = true;        // boolean
let rate = 128.5;          // float
```

### Arrays

```rhai
let stations = [];
stations.push("one");
stations.push("two");
stations.len();            // 2
stations[0];               // "one"
```

### Object Maps  (`#{ }`)

Use `#{ key: value }` — similar to JSON objects:

```rhai
let station = #{
    "name": "Groove Salad",
    "url": "https://example.com/stream",
    "tags": "ambient,chill"
};
station.name;              // "Groove Salad"  (dot access)
station["url"];            // "https://example.com/stream"  (bracket access)
```

### Loops

```rhai
// For loop over array
for item in items {
    print(`item`);
}

// While loop
let i = 0;
while i < 10 {
    i += 1;
}
```

### If / Else

```rhai
if name == "" {
    name = "Unknown";
}
```

### String Interpolation

Backtick strings with `${ }`:

```rhai
let url = `https://somafm.com/${slug}/`;
```

### Try / Catch

```rhai
let result = try http_get_json(url) catch {
    // fallback — return empty array
    []
};
```

### print / debug

```rhai
print("hello");            // stdout (visible in CLI mode)
debug("some value");       // stderr
```

These are useful while developing a script in the CLI. In GUI mode output is
captured by the engine.

---

## Available Functions

### `http_get(url)` -> `String`

Fetches the body of a URL as a string. Good for scraping HTML.

```rhai
let html = http_get("https://somafm.com/");
```

- Timeout: 10 seconds
- Throws a runtime error if the request fails

### `http_get_json(url)` -> `Dynamic`

Fetches a URL and parses the response as JSON, returning a Rhai value
(arrays, maps, strings, numbers, booleans).

```rhai
let data = http_get_json("https://api.radio-browser.info/json/stations?limit=10");

// data is now a Rhai array of maps
for station in data {
    print(station.name);
}
```

- Throws a runtime error if the request fails or the response is not valid JSON

### `html_select(html, css)` -> `String` (JSON)

Parses an HTML string and runs a CSS selector against it. Returns a JSON
**string** that you parse with `json_parse`.

```rhai
let html = http_get("https://somafm.com/");
let links_json = html_select(html, "a[href^='/player24/station/']");
let links = json_parse(links_json);
```

Each matched element is a map with:

| Field   | Description                                        |
| ------- | -------------------------------------------------- |
| `tag`   | Element name (e.g. `"a"`, `"img"`)                |
| `attrs` | Map of HTML attributes (`#{ "href": "...", ... }`) |
| `text`  | Concatenated text content                          |
| `html`  | Inner HTML of the element                          |

```rhai
for el in links {
    let href = el.attrs["href"];
    let text = el.text;
}
```

- Uses the `scraper` crate (a Rust library that mirrors the browser CSS
  Selectors API)
- Throws on invalid CSS syntax

### `json_parse(string)` -> `Dynamic`

Parses a JSON string into a Rhai value (array, map, string, number, boolean,
or null).

```rhai
let val = json_parse(`{"a": 1, "b": "hello"}`);
val.a;  // 1
```

### `json_stringify(value)` -> `String`

Converts any Rhai value back into a JSON string. **Every discovery script
must end with `json_stringify(...)`** — the runner expects the script's
result to be a JSON string.

```rhai
json_stringify(result)
```

---

## Output Format

The script must return a JSON object with this shape (defined as
`DiscoveryResult` in Rust):

```json
{
  "provider": "MyProvider",
  "stations": [
    {
      "name": "Station Name",
      "stream_url": "https://...",
      "stream_urls": [
        {
          "url": "https://...",
          "codec": "MP3",
          "bitrate": 256,
          "tls": true
        }
      ],
      "homepage": "https://...",
      "icon_url": "https://...",
      "tags": "genre,style",
      "country": "US",
      "language": "en"
    }
  ]
}
```

### Field Reference

| Field          | Type                | Required | Description                                |
| -------------- | ------------------- | -------- | ------------------------------------------ |
| `provider`     | `String`            | **yes**  | Display name for the source                |
| `stations`     | `Array<Station>`    | **yes**  | List of discovered stations                |
| `name`         | `String`            | **yes**  | Station display name                       |
| `stream_url`   | `String`            | **yes**  | Primary stream URL (for quick playback)     |
| `stream_urls`  | `Array<StreamUrl>`  | no       | Alternative URL / codec / bitrate variants |
| `.url`         | `String`            | **yes**  | Stream URL                                 |
| `.codec`       | `String` or `null`  | no       | Codec name (e.g. `"MP3"`, `"AAC"`)        |
| `.bitrate`     | `Integer` or `null` | no       | Bitrate in kbps (e.g. `128`, `256`)        |
| `.tls`         | `Boolean` or `null` | no       | Whether the stream uses HTTPS/TLS          |
| `homepage`     | `String` or `null`  | no       | Station website URL                        |
| `icon_url`     | `String` or `null`  | no       | Station logo / favicon URL                 |
| `tags`         | `String` or `null`  | no       | Comma-separated tags or genres             |
| `country`      | `String` or `null`  | no       | ISO 3166-1 alpha-2 country code            |
| `language`     | `String` or `null`  | no       | Language code (e.g. `"en"`, `"de"`)        |

Only `name` and `stream_url` are required per station. All other fields are
optional but recommended for a better user experience.

---

## Error Handling

### In the script

Wrap fallible calls in `try / catch` to handle failures gracefully:

```rhai
let data = try http_get_json("https://api.example.com/stations") catch {
    // Return empty result on network error
    json_stringify(#{
        "provider": "Example",
        "stations": []
    })
};
```

Without `try / catch`, any error (network failure, JSON parse error, CSS
selector syntax error) will abort the script and the runner will report the
error.

### From the runner

The runner reports errors with clear messages:

```
Error: compile error: ...         (syntax error in the script)
Error: runtime error: ...         (network failure, division by zero, etc.)
Error: parse error: ...           (returned JSON doesn't match DiscoveryResult)
Error: script timed out           (execution exceeded the timeout limit)
```

### Sandbox Limits

| Limit                 | Value      |
| --------------------- | ---------- |
| Max operations        | 500,000    |
| Max call depth        | 50         |
| Max string size       | 10 MB      |
| HTTP timeout          | 10 seconds |
| Script timeout (CLI)  | 30 seconds |

If a script exceeds these limits, the engine terminates it and reports an
error.

---

## Testing with the CLI

The `shortwave-script` CLI tool lets you run, list, and validate scripts
without launching the GUI.

### Run a script

```bash
cargo run --bin shortwave-script -- run path/to/script.rhai
```

### Run with pretty-printed output

```bash
cargo run --bin shortwave-script -- run path/to/script.rhai --pretty
```

Example output:

```
Provider: SomaFM
Stations: 37

1. Groove Salad
   URL: https://somafm.com/groovesalad.pls
   MP3 | 256 kbps | TLS
   Web: https://somafm.com/groovesalad/
   Tags: soma fm,electronic,independent
   Country: US

2. Fluid
   ...
```

### List registered providers

```bash
cargo run --bin shortwave-script -- list
```

### Validate a script (compile + run + parse check)

```bash
cargo run --bin shortwave-script -- validate path/to/script.rhai
```

Validating checks three things:

1. The script compiles (no syntax errors)
2. It executes without runtime errors
3. The return value deserialises to a valid `DiscoveryResult`

---

## Best Practices

1. **Always use try/catch around HTTP calls.** Network requests can fail for
   many reasons. Return an empty stations array rather than crashing.

2. **Deduplicate stations.** When scraping HTML, the same station may appear
   in multiple rows. Track seen station IDs in a Rhai map (`seen = #{}`) and
   `continue` on duplicates:

   ```rhai
   let seen = #{};
   for link in links {
       let slug = extract_id(link);
       if seen.contains(slug) { continue; }
       seen[slug] = true;
       // ... build station entry
   }
   ```

3. **Prefer `http_get_json` for APIs, `http_get` + `html_select` for HTML.**
   JSON APIs are more stable than scraping HTML. Only scrape when no API
   exists.

4. **Include `stream_urls` for multi-format stations.** Provide all known
   quality / codec variants so the user gets the best match.

5. **Keep scripts simple.** A discovery script should be 10-70 lines. If yours
   is longer, consider whether the logic could be simpler.

6. **Set a meaningful `provider` name.** This is shown in the UI as the source
   of the stations.

7. **Validate before shipping.** Always run `shortwave-script validate` on
   your script to catch issues early.

---

## Example: Minimal Script

```rhai
// data/examples/hello-world.rhai

json_stringify(#{
    "provider": "Hello World",
    "stations": [
        #{
            "name": "Public Domain Jazz",
            "stream_url": "https://example.com/jazz.ogg",
            "stream_urls": [
                #{ "url": "https://example.com/jazz.ogg", "codec": "OGG", "bitrate": 192, "tls": true }
            ],
            "tags": "jazz,public domain"
        }
    ]
})
```

## Example: HTML Scraper

```rhai
// data/examples/somafm-simple.rhai

let html = http_get("https://somafm.com/");
let links_json = html_select(html, "a[href^='/player24/station/']");
let links = json_parse(links_json);

let seen = #{};
let stations = [];

for link in links {
    let slug = link.attrs["href"].split("/")[3];

    if seen.contains(slug) { continue; }
    seen[slug] = true;

    // Grab the img inside the link for the channel name
    let children_json = html_select(link.html, "img");
    let children = json_parse(children_json);
    let name = children.len() > 0 ? children[0].attrs["alt"].split(":")[0] : slug;

    stations.push(#{
        "name": name,
        "stream_url": `https://somafm.com/${slug}.pls`,
        "stream_urls": [
            #{ "url": `https://somafm.com/${slug}.pls`, "codec": "MP3", "bitrate": 256, "tls": true },
            #{ "url": `https://somafm.com/${slug}128.pls`, "codec": "MP3", "bitrate": 128, "tls": true }
        ],
        "homepage": `https://somafm.com/${slug}/`,
        "tags": "soma fm,electronic,independent",
        "country": "US"
    });
}

json_stringify(#{
    "provider": "SomaFM",
    "stations": stations
})
```

## Example: JSON API

```rhai
// data/examples/radio-browser.rhai

let result = try http_get_json("https://at1.api.radio-browser.info/json/stations?limit=100") catch {
    json_stringify(#{
        "provider": "Radio-Browser.info",
        "stations": []
    })
};

let stations = [];

for station in result {
    let urls = [];
    if station.url != "" {
        urls.push(#{
            "url": station.url,
            "codec": station.codec,
            "bitrate": station.bitrate,
            "tls": false
        });
    }

    stations.push(#{
        "name": station.name,
        "stream_url": station.url,
        "stream_urls": urls,
        "homepage": station.homepage,
        "icon_url": station.favicon,
        "tags": station.tags,
        "country": station.country,
        "language": station.language
    });
}

json_stringify(#{
    "provider": "Radio-Browser.info",
    "stations": stations
})
```
