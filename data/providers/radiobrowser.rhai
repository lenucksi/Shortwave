// Discovery-Provider: Radio-Browser.info
// Description: Große öffentliche DB mit Tausenden Radiosendern

let url = "https://all.api.radio-browser.info/json/stations?limit=100";

let raw = [];
try {
   raw = http_get_json(url);
} catch (error) {
   // API nicht erreichbar, leeres Ergebnis zurückgeben
};

let results = [];
for station in raw {
   let stream_url = if station.url == () { "" } else { station.url };
   let urls = [];
   if stream_url != "" {
      urls.push(#{
         "url": station.url,
         "codec": station.codec,
         "bitrate": station.bitrate,
         "tls": false
      });
   }

   results.push(#{
      "name": station.name,
      "stream_url": stream_url,
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
   "stations": results
})
