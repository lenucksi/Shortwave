// Discovery-Provider: SomaFM
// Description: Simplified scraper — minimal format list, two quality levels

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
    let name = slug;
    let icon_url = "";
    if children.len() > 0 {
        name = children[0].attrs["alt"].split(":")[0];
        icon_url = "https://somafm.com" + children[0].attrs["src"];
    }

    stations.push(#{
        "name": name,
        "stream_url": `https://somafm.com/${slug}.pls`,
        "stream_urls": [
            #{ "url": `https://somafm.com/${slug}.pls`, "codec": "MP3", "bitrate": 256, "tls": true },
            #{ "url": `https://somafm.com/${slug}128.pls`, "codec": "MP3", "bitrate": 128, "tls": true },
            #{ "url": `https://somafm.com/${slug}130.pls`, "codec": "AAC", "bitrate": 128, "tls": true }
        ],
        "homepage": `https://somafm.com/${slug}/`,
        "icon_url": icon_url,
        "tags": "soma fm,electronic,independent",
        "country": "US",
        "language": "en"
    });
}

json_stringify(#{
    "provider": "SomaFM",
    "stations": stations
})
