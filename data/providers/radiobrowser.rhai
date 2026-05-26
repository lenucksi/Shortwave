// Discovery-Provider: Radio-Browser.info
// Description: Große öffentliche DB mit Tausenden Radiosendern

let url = "https://at1.api.radio-browser.info/json/stations?limit=100";

let result = try http_get_json(url) catch {
   json_stringify(#{
      "provider": "Radio-Browser.info",
      "stations": []
   })
};

let stations = [];

for station in result {
   let stream_url = station.url;
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
   "stations": stations
})
