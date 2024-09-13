use tidal_rs::api::Page;

fn main() {
  let de = &mut serde_json::Deserializer::from_str(JSON);
  let result: Result<Page, _> = serde_path_to_error::deserialize(de);

  println!("{:#?}", result);
}

const JSON: &str = r##"{
  "selfLink": null,
  "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjR9",
  "title": "Gas",
  "rows": [
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiJiZTE5ZGY4Ni00NDE3LTQ5ZWYtOGVjNC0xZTNjMDk3NTA3ZGEiLCJtViI6MSwibUgiOiI0NWMyZjJjYSJ9",
          "type": "ARTIST_HEADER",
          "width": 100,
          "title": "",
          "description": "",
          "preTitle": "",
          "artist": {
            "id": 4279312,
            "name": "Gas",
            "url": "http://www.tidal.com/artist/4279312",
            "picture": null,
            "artistTypes": [
              "ARTIST",
              "CONTRIBUTOR"
            ],
            "mixes": {
              "ARTIST_MIX": "0008e5737fd2215095481acf31ab1d"
            }
          },
          "bio": {
            "text": null,
            "source": null
          },
          "store": null,
          "artistMix": {
            "id": "0008e5737fd2215095481acf31ab1d"
          },
          "roleCategories": null,
          "playbackControls": [
            {
              "shuffle": true,
              "playbackMode": "SHUFFLE",
              "title": "Shuffle",
              "icon": "shuffle_tracks",
              "targetModuleId": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiI4ODllN2MyYS1lM2JlLTQ3MmItOTgwMS1iZjZiNDA2NTUwNTAiLCJtViI6MSwibUgiOiJjY2ExYTZiMiJ9"
            },
            {
              "shuffle": false,
              "playbackMode": "PLAY",
              "title": "Play",
              "icon": "play_tracks",
              "targetModuleId": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiI4ODllN2MyYS1lM2JlLTQ3MmItOTgwMS1iZjZiNDA2NTUwNTAiLCJtViI6MSwibUgiOiJjY2ExYTZiMiJ9"
            }
          ],
          "mixes": {
            "ARTIST_MIX": "0008e5737fd2215095481acf31ab1d"
          }
        }
      ]
    },
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiI4ODllN2MyYS1lM2JlLTQ3MmItOTgwMS1iZjZiNDA2NTUwNTAiLCJtViI6MSwibUgiOiJjY2ExYTZiMiJ9",
          "type": "TRACK_LIST",
          "width": 100,
          "title": "Top Tracks",
          "description": "",
          "preTitle": null,
          "showMore": {
            "title": "View all",
            "apiPath": "pages/single-module-page/ae223310-a4c2-4568-a770-ffef70344441/4/889e7c2a-e3be-472b-9801-bf6b40655050/1?artistId=4279312"
          },
          "supportsPaging": false,
          "quickPlay": false,
          "listFormat": "NUMBERS",
          "scroll": null,
          "layout": null,
          "pagedList": {
            "limit": 4,
            "offset": 0,
            "totalNumberOfItems": 300,
            "items": [
              {
                "id": 34398218,
                "title": "Microscopic",
                "duration": 594,
                "version": null,
                "url": "https://tidal.com/browse/track/34398218",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "album": {
                  "id": 34398215,
                  "title": "Gas 0095",
                  "cover": "a6ff9fe4-9684-4b17-872c-2a6c0ca05465",
                  "vibrantColor": "#af2d2d",
                  "videoCover": null,
                  "url": "https://tidal.com/browse/album/34398215",
                  "releaseDate": "2008-06-27"
                },
                "explicit": false,
                "volumeNumber": 1,
                "trackNumber": 3,
                "popularity": 4,
                "doublePopularity": 0.04026840061740821,
                "allowStreaming": true,
                "streamReady": true,
                "streamStartDate": "2007-08-12T00:00:00.000+0000",
                "adSupportedStreamReady": true,
                "djReady": true,
                "stemReady": false,
                "editable": false,
                "replayGain": -5.1,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "mixes": {
                  "TRACK_MIX": "0011772cf1cc372cd7fd0655274383"
                },
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 34398217,
                "title": "Experiments on Live Electricity",
                "duration": 1000,
                "version": null,
                "url": "https://tidal.com/browse/track/34398217",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "album": {
                  "id": 34398215,
                  "title": "Gas 0095",
                  "cover": "a6ff9fe4-9684-4b17-872c-2a6c0ca05465",
                  "vibrantColor": "#af2d2d",
                  "videoCover": null,
                  "url": "https://tidal.com/browse/album/34398215",
                  "releaseDate": "2008-06-27"
                },
                "explicit": false,
                "volumeNumber": 1,
                "trackNumber": 2,
                "popularity": 9,
                "doublePopularity": 0.08770711003093377,
                "allowStreaming": true,
                "streamReady": true,
                "streamStartDate": "2007-08-12T00:00:00.000+0000",
                "adSupportedStreamReady": true,
                "djReady": true,
                "stemReady": false,
                "editable": false,
                "replayGain": -5.1,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "mixes": {
                  "TRACK_MIX": "001fb129b7e0551300931d99d4fcbf"
                },
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 26521433,
                "title": "Get Ya Freak on (feat. Gas)",
                "duration": 270,
                "version": null,
                "url": "https://tidal.com/browse/track/26521433",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "album": {
                  "id": 26521424,
                  "title": "Sextape",
                  "cover": "636fd2d4-5b78-4f8e-ae2f-f1fcf68b149b",
                  "vibrantColor": "#fcded3",
                  "videoCover": null,
                  "url": "https://tidal.com/browse/album/26521424",
                  "releaseDate": "2010-01-01"
                },
                "explicit": false,
                "volumeNumber": 1,
                "trackNumber": 9,
                "popularity": 9,
                "doublePopularity": 0.08557613574580704,
                "allowStreaming": true,
                "streamReady": true,
                "streamStartDate": "2014-02-12T00:00:00.000+0000",
                "adSupportedStreamReady": true,
                "djReady": true,
                "stemReady": false,
                "editable": false,
                "replayGain": -11.36,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "mixes": {
                  "TRACK_MIX": "0012d5b7e81d360759c16a48aa7840"
                },
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 194254168,
                "title": "Nach 1912 [Mixed]",
                "duration": 422,
                "version": null,
                "url": "https://tidal.com/browse/track/194254168",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "album": {
                  "id": 194254157,
                  "title": "Connecting The Dots (DJ Mix)",
                  "cover": "0c8b9d72-e1ce-4184-9288-160d88ad4c45",
                  "vibrantColor": "#f96a1b",
                  "videoCover": null,
                  "url": "https://tidal.com/browse/album/194254157",
                  "releaseDate": "2021-09-17"
                },
                "explicit": false,
                "volumeNumber": 1,
                "trackNumber": 11,
                "popularity": 3,
                "doublePopularity": 0.03465427384021066,
                "allowStreaming": true,
                "streamReady": true,
                "streamStartDate": "2021-09-17T00:00:00.000+0000",
                "adSupportedStreamReady": true,
                "djReady": true,
                "stemReady": false,
                "editable": false,
                "replayGain": -4.87,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "mixes": {
                  "TRACK_MIX": "0010568d3a60cacdaf5e5deb789f32"
                },
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              }
            ],
            "dataApiPath": "pages/data/25b47120-6a2f-4dbb-8a38-daa415367d22?artistId=4279312"
          },
          "showTableHeaders": true
        }
      ]
    },
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiJhNGY5NjRiYS1iNTJlLTQxZTgtYjI1Yy0wNmNkNzBjMWVmYWQiLCJtViI6MiwibUgiOiJhZDZlYzE2MSJ9",
          "type": "ALBUM_LIST",
          "width": 100,
          "title": "Albums",
          "description": "",
          "preTitle": null,
          "showMore": {
            "title": "View all",
            "apiPath": "pages/single-module-page/ae223310-a4c2-4568-a770-ffef70344441/4/a4f964ba-b52e-41e8-b25c-06cd70c1efad/2?artistId=4279312"
          },
          "supportsPaging": true,
          "quickPlay": false,
          "scroll": "HORIZONTAL",
          "listFormat": null,
          "pagedList": {
            "limit": 50,
            "offset": 0,
            "totalNumberOfItems": 22,
            "items": [
              {
                "id": 371443744,
                "title": "GAS",
                "cover": "2165848f-9961-4824-95af-c760c0f332f8",
                "vibrantColor": "#fbf25e",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/371443744",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-08-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 6,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-08-23",
                "duration": 5576,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 245001402,
                "title": "Dancehall Mafia 1.0",
                "cover": "9db13e46-4167-468d-8c68-71546a2dc234",
                "vibrantColor": "#e49fa0",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/245001402",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2022-08-24T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 9,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-08-16",
                "duration": 1614,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 242720962,
                "title": "Breck N' Be",
                "cover": "ec9ed889-a4e4-448a-a7f1-1a8d707801d0",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/242720962",
                "artists": [
                  {
                    "id": 3659127,
                    "name": "Braille",
                    "type": "MAIN",
                    "picture": "ba805747-e83d-4e8a-9c6c-7ba3ec6682e7"
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-04-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 6,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-06-24",
                "duration": 823,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 209367832,
                "title": "GAS",
                "cover": "03668fa7-83eb-40ed-9b0d-989f8ff1fcc7",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/209367832",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-01-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 10,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-01-20",
                "duration": 2175,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 272124773,
                "title": "PENSIERI",
                "cover": "41d18312-e0ea-4596-9065-88f88a3ff97a",
                "vibrantColor": "#878feb",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/272124773",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 16807653,
                    "name": "Bleda",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2020-12-28T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 11,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-12-28",
                "duration": 1892,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 154016029,
                "title": "Old & New",
                "cover": "ca1b0ec6-581f-4bb4-9438-97de50073f26",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/154016029",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2020-09-18T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 6,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-09-18",
                "duration": 1210,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 144366408,
                "title": "Aqua Gym Latin Hits 2020 Workout Collection (15 Tracks Non-Stop Mixed Compilation for Fitness & Workout - 128 Bpm / 32 Count)",
                "cover": "4a0a733c-62a2-4579-841f-0d5a9b622ab0",
                "vibrantColor": "#93e5f2",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/144366408",
                "artists": [
                  {
                    "id": 3603975,
                    "name": "Caruso",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7143079,
                    "name": "Valenziano",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5182482,
                    "name": "Kros",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5814753,
                    "name": "Loga Y Crucas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 6242310,
                    "name": "Miki M",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4618360,
                    "name": "Comis",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3987321,
                    "name": "Silvano Del Gado",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5283510,
                    "name": "Andrea Paci",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5300089,
                    "name": "Proyecto Loco",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4608945,
                    "name": "Mariucch",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5010432,
                    "name": "Santiago El Don",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7351719,
                    "name": "PNP",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5300088,
                    "name": "Bom Dia",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4285905,
                    "name": "Relight Orchestra",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-06-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-06-10",
                "duration": 3470,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 144360327,
                "title": "Top Songs For Street Workout Latin Hits 2020 Fitness Compilation",
                "cover": "2b83cdfb-88be-4310-af24-93bf5dc2f9ea",
                "vibrantColor": "#e6a1d5",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/144360327",
                "artists": [
                  {
                    "id": 5182482,
                    "name": "Kros",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 6242310,
                    "name": "Miki M",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5300088,
                    "name": "Bom Dia",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4285905,
                    "name": "Relight Orchestra",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 32474,
                    "name": "Desaparecidos",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4608945,
                    "name": "Mariucch",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4047987,
                    "name": "DREAMERS",
                    "type": "MAIN",
                    "picture": "ff963c52-6925-49dc-a8dc-73537a3559b7"
                  },
                  {
                    "id": 5300089,
                    "name": "Proyecto Loco",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3603975,
                    "name": "Caruso",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7143079,
                    "name": "Valenziano",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4618360,
                    "name": "Comis",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5814753,
                    "name": "Loga Y Crucas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3987321,
                    "name": "Silvano Del Gado",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5283510,
                    "name": "Andrea Paci",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9682635,
                    "name": "Familia Latina",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4020973,
                    "name": "MJ",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-06-08T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-06-08",
                "duration": 3815,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 140289985,
                "title": "Most Rated Singapore EDM Music 2020 Session",
                "cover": "06d5a0a6-57d4-42d5-b9e8-a8b8f295943e",
                "vibrantColor": "#aed8e7",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/140289985",
                "artists": [
                  {
                    "id": 6273354,
                    "name": "Andrea Di Pietro",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 12878,
                    "name": "Zen",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716514,
                    "name": "The Clubbers",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7957932,
                    "name": "Paul Zak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 6044195,
                    "name": "M&Project",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5011905,
                    "name": "Two at work",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7618843,
                    "name": "Francesco Ienco",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3639004,
                    "name": "Raf Marchesini",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7495133,
                    "name": "Adrian Alter",
                    "type": "MAIN",
                    "picture": "10abce6a-56ca-48b0-b8f6-9d9ebcafb6f1"
                  },
                  {
                    "id": 9569132,
                    "name": "Jowel Cole",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5958359,
                    "name": "Lanfranchi",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4932133,
                    "name": "Farina",
                    "type": "MAIN",
                    "picture": "b8999a86-8971-4f7b-9743-1180ff45955a"
                  },
                  {
                    "id": 3500261,
                    "name": "Federico Franchi",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3711977,
                    "name": "Provenzano",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7618842,
                    "name": "Monosono",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716513,
                    "name": "Acdf",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716511,
                    "name": "Des3Ett",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-05-08T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-05-08",
                "duration": 4188,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 135662526,
                "title": "Singapore Latin 2020 Session",
                "cover": "54b63104-8048-47c7-b347-50e71b0565a6",
                "vibrantColor": "#c1d25c",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/135662526",
                "artists": [
                  {
                    "id": 5300088,
                    "name": "Bom Dia",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4618360,
                    "name": "Comis",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4285905,
                    "name": "Relight Orchestra",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4455048,
                    "name": "Sapienza",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3735983,
                    "name": "Dago",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7029572,
                    "name": "Funkyman",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9682638,
                    "name": "DJ Chama",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7677107,
                    "name": "Julio Cesar El Emperador",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 16244,
                    "name": "Los Locos",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4659386,
                    "name": "El 3mendo",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5300089,
                    "name": "Proyecto Loco",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3603975,
                    "name": "Caruso",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7143079,
                    "name": "Valenziano",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4608945,
                    "name": "Mariucch",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4047987,
                    "name": "DREAMERS",
                    "type": "MAIN",
                    "picture": "ff963c52-6925-49dc-a8dc-73537a3559b7"
                  },
                  {
                    "id": 5814753,
                    "name": "Loga Y Crucas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5182482,
                    "name": "Kros",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3987321,
                    "name": "Silvano Del Gado",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5283510,
                    "name": "Andrea Paci",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7351719,
                    "name": "PNP",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9682635,
                    "name": "Familia Latina",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4020973,
                    "name": "MJ",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-04-03T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-04-03",
                "duration": 3687,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 134228122,
                "title": "Gods Of Dublin Latin Tracks",
                "cover": "7625bc6f-09f3-47ad-a067-983364e51211",
                "vibrantColor": "#d5d597",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/134228122",
                "artists": [
                  {
                    "id": 3603975,
                    "name": "Caruso",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7143079,
                    "name": "Valenziano",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9682635,
                    "name": "Familia Latina",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4020973,
                    "name": "MJ",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4608945,
                    "name": "Mariucch",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4047987,
                    "name": "DREAMERS",
                    "type": "MAIN",
                    "picture": "ff963c52-6925-49dc-a8dc-73537a3559b7"
                  },
                  {
                    "id": 7351719,
                    "name": "PNP",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3987321,
                    "name": "Silvano Del Gado",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5283510,
                    "name": "Andrea Paci",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5182482,
                    "name": "Kros",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 6242310,
                    "name": "Miki M",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5814753,
                    "name": "Loga Y Crucas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 32474,
                    "name": "Desaparecidos",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4285905,
                    "name": "Relight Orchestra",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5300088,
                    "name": "Bom Dia",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4618360,
                    "name": "Comis",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-03-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-03-20",
                "duration": 3717,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 133527843,
                "title": "Mega Dublin EDM Hits 2020 Session",
                "cover": "b1cb58fb-35c4-475b-b6c6-7b3d68287798",
                "vibrantColor": "#8c8f1d",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/133527843",
                "artists": [
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9763795,
                    "name": "Ubig",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7480314,
                    "name": "Apple DJ's",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9651121,
                    "name": "Incatasciato",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5855361,
                    "name": "Mark'S M",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3639004,
                    "name": "Raf Marchesini",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5252444,
                    "name": "Karim Razak",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716511,
                    "name": "Des3Ett",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716514,
                    "name": "The Clubbers",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7716513,
                    "name": "Acdf",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 9569132,
                    "name": "Jowel Cole",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 7495133,
                    "name": "Adrian Alter",
                    "type": "MAIN",
                    "picture": "10abce6a-56ca-48b0-b8f6-9d9ebcafb6f1"
                  },
                  {
                    "id": 6273354,
                    "name": "Andrea Di Pietro",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 12878,
                    "name": "Zen",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 8163394,
                    "name": "C&V",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5855363,
                    "name": "Nigo'",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 3544213,
                    "name": "The Dreamers",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5855362,
                    "name": "Breakers Dj'S",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 5958359,
                    "name": "Lanfranchi",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4932133,
                    "name": "Farina",
                    "type": "MAIN",
                    "picture": "b8999a86-8971-4f7b-9743-1180ff45955a"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-03-13T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-03-13",
                "duration": 4150,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 133673276,
                "title": "Vórtice",
                "cover": "d84654a2-885c-4461-a517-b175e1e2dc55",
                "vibrantColor": "#8ab5d2",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/133673276",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-03-06T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 8,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-03-06",
                "duration": 2734,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 132034688,
                "title": "4 EVER",
                "cover": "601e4c98-5d96-413d-8e66-4b5875a99d0a",
                "vibrantColor": "#c77a88",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/132034688",
                "artists": [
                  {
                    "id": 3529836,
                    "name": "Peach",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 16807653,
                    "name": "Bleda",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2020-02-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 8,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-02-23",
                "duration": 1171,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 93310241,
                "title": "Electriphonico",
                "cover": "1b0f0cf5-9d79-4c33-b3a7-3c760a5222f5",
                "vibrantColor": "#f3a63b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/93310241",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2018-07-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 6,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2018-07-23",
                "duration": 2154,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 210797762,
                "title": "Demoralisasi Penguasa Bangsa",
                "cover": "79221fd7-e7e5-4528-9a41-8aa8116b1364",
                "vibrantColor": "#ea1315",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/210797762",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-03-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 9,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-03-01",
                "duration": 1438,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 16103052,
                "title": "Lieben / Hassen (10th Anniversary)",
                "cover": "969d8be4-d29a-4277-9f43-d90d8ef44863",
                "vibrantColor": "#e9d679",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/16103052",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2012-08-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2012-06-24",
                "duration": 2558,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 34398215,
                "title": "Gas 0095",
                "cover": "a6ff9fe4-9684-4b17-872c-2a6c0ca05465",
                "vibrantColor": "#af2d2d",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/34398215",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2007-08-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2008-06-27",
                "duration": 4165,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 23214609,
                "title": "Oriental Mood",
                "cover": "a15e6845-4706-4f50-a8e2-240ffd148baf",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/23214609",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2008-05-28T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 10,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2008-01-01",
                "duration": 2664,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 87239812,
                "title": "Neue Zeit",
                "cover": "473cfc6c-d3f6-47b0-9ac3-8570909fe03f",
                "vibrantColor": "#f0e83b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/87239812",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2018-05-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 11,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2007-12-13",
                "duration": 2365,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 106950865,
                "title": "Great American Songs",
                "cover": "48678cb4-72a6-4346-9641-eeb8e44e3ce6",
                "vibrantColor": "#9dafd9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/106950865",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-04-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 10,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "1999-11-15",
                "duration": 4129,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 235543733,
                "title": "GAS",
                "cover": "a5032865-5329-4403-b448-4f64fee1d04b",
                "vibrantColor": "#f2bc66",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/235543733",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "1990-11-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 11,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "1990-11-01",
                "duration": 2298,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              }
            ],
            "dataApiPath": "pages/data/4b37c74b-f994-45dd-8fca-b7da2694da83?artistId=4279312"
          },
          "header": null,
          "layout": null
        }
      ]
    },
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiI2NWYwOTU0Ny0zYmJhLTQ3NjQtYWQwYy04ZTEwNWY3NzZmYTciLCJtViI6MSwibUgiOiJkZTA1YWZiYyJ9",
          "type": "ALBUM_LIST",
          "width": 100,
          "title": "EP & Singles",
          "description": "",
          "preTitle": null,
          "showMore": {
            "title": "View all",
            "apiPath": "pages/single-module-page/ae223310-a4c2-4568-a770-ffef70344441/4/65f09547-3bba-4764-ad0c-8e105f776fa7/1?artistId=4279312"
          },
          "supportsPaging": true,
          "quickPlay": false,
          "scroll": "HORIZONTAL",
          "listFormat": null,
          "pagedList": {
            "limit": 50,
            "offset": 0,
            "totalNumberOfItems": 114,
            "items": [
              {
                "id": 381084524,
                "title": "liberte",
                "cover": "4f1f2943-03d3-486f-aa0a-3a07dd7fc300",
                "vibrantColor": "#b3e2ff",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/381084524",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-08-13T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-08-13",
                "duration": 52,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 380031285,
                "title": "4R3R",
                "cover": "7278b90d-84fd-48bf-baf2-bce6d8348cd3",
                "vibrantColor": "#3d5d8b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/380031285",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-08-08T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-08-08",
                "duration": 194,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 381864355,
                "title": "开火Freestyle",
                "cover": "0254d50e-4f7f-413b-9e62-439ba0118baa",
                "vibrantColor": "#e0897a",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/381864355",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-08-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-08-06",
                "duration": 206,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 374722648,
                "title": "STYLE 2",
                "cover": "bc100ebe-e820-4e5e-9ee5-e84a40853c60",
                "vibrantColor": "#c2798f",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/374722648",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-07-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-07-11",
                "duration": 151,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 366977629,
                "title": "style 1",
                "cover": "22c67841-8d23-47b4-9041-2983d4f9d467",
                "vibrantColor": "#a3d2d9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/366977629",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-06-02T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-06-02",
                "duration": 109,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 372314063,
                "title": "EL ARTE",
                "cover": "1faa00f5-e35f-4432-8d0c-f207068d3502",
                "vibrantColor": "#f6c88c",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/372314063",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-06-29T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-05-14",
                "duration": 146,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 364396898,
                "title": "PURE TRUTH",
                "cover": "d4df4b51-ea6e-4736-8bc9-8d689cad2abb",
                "vibrantColor": "#eccb84",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/364396898",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-05-14T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-05-14",
                "duration": 163,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 357245225,
                "title": "Giovanni",
                "cover": "e886468f-fc61-4641-acd0-c024598dae62",
                "vibrantColor": "#9cbae1",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/357245225",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-04-26T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-04-26",
                "duration": 107,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 353971571,
                "title": "Pisokouna",
                "cover": "08adb6b2-f9fe-4e8f-8963-d526cd19b31d",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/353971571",
                "artists": [
                  {
                    "id": 4192750,
                    "name": "Sugar Boy",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4123121,
                    "name": "Teez",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-03-29T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-03-29",
                "duration": 122,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 352457046,
                "title": "Je Suis Confiné",
                "cover": "becf439f-40cd-4873-b59d-3475d2cbcfc7",
                "vibrantColor": "#40bf99",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/352457046",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-03-22T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 4,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-03-22",
                "duration": 983,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 351245709,
                "title": "OH MA BELLAA",
                "cover": "dbb676d4-ae15-4e9f-ace0-b88ffb446b24",
                "vibrantColor": "#d8a9b8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/351245709",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-03-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-03-11",
                "duration": 137,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 346097745,
                "title": "Caja negra",
                "cover": "427ee88d-d1a0-43c4-a3fd-b06f1135400c",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/346097745",
                "artists": [
                  {
                    "id": 29292373,
                    "name": "Sastre.",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-03-06T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-03-06",
                "duration": 204,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 344055136,
                "title": "Nursery",
                "cover": "d8af8117-ed3e-4b01-82c4-cf70ebb26080",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/344055136",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-03-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-03-01",
                "duration": 251,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 343739719,
                "title": "Sbalzi d'umore",
                "cover": "7c9ced2c-7ab0-4c44-b20f-5a336908f643",
                "vibrantColor": "#7850bc",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/343739719",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 16168305,
                    "name": "Yung Da6",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-02-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-02-23",
                "duration": 164,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 341520272,
                "title": "Queens of Hell",
                "cover": "07c8a152-ac7c-41a7-9597-498a3b395f4a",
                "vibrantColor": "#d098a7",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/341520272",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-01-25T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-01-24",
                "duration": 60,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 339220658,
                "title": "People Say",
                "cover": "c5064f62-5e74-48f7-9ecb-3ab84ba684c0",
                "vibrantColor": "#3d6b96",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/339220658",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2024-01-19T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2024-01-19",
                "duration": 367,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 334950877,
                "title": "Tierra Moja'",
                "cover": "51db5622-6358-4e8a-943d-2e629b9d05d4",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/334950877",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-12-13T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-12-13",
                "duration": 165,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 334406546,
                "title": "Volando",
                "cover": "958ae306-a9f2-4413-bb06-c9e6d07d710c",
                "vibrantColor": "#e9b9b9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/334406546",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-12-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-12-10",
                "duration": 211,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 334200688,
                "title": "REKUP",
                "cover": "329415cd-776c-4066-8327-3b977259f316",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/334200688",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-12-08T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-12-08",
                "duration": 95,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 328809686,
                "title": "COMO",
                "cover": "92c4703e-918f-4437-b49f-9d718a9dea1c",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/328809686",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-11-14T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-11-14",
                "duration": 165,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 323093225,
                "title": "PROBLEMI",
                "cover": "c8bc45aa-da51-4c15-996c-ed910ecdd470",
                "vibrantColor": "#deb956",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/323093225",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 16168305,
                    "name": "Yung Da6",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-11-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-11-10",
                "duration": 179,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 314418857,
                "title": "Famous",
                "cover": "a8e042b9-f9a9-480c-9a1b-92e22afcf68c",
                "vibrantColor": "#e76a6a",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/314418857",
                "artists": [
                  {
                    "id": 5723278,
                    "name": "Filey",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-10-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-10-01",
                "duration": 149,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 315880096,
                "title": "EL PERFUME (feat. Size)",
                "cover": "3d987de6-5fc4-43fc-a753-e0765cf75d93",
                "vibrantColor": "#b6cddc",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/315880096",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-09-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-09-11",
                "duration": 163,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 310911844,
                "title": "VIVIR SIN TI",
                "cover": "55d90dfd-b684-4baf-9316-7a705eb24b22",
                "vibrantColor": "#5393fd",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/310911844",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-08-25T10:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-08-25",
                "duration": 148,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 297529289,
                "title": "DESCONOCIDOS",
                "cover": "ff6c31a9-7941-4454-8ab6-ca9bbbf68329",
                "vibrantColor": "#066ccd",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/297529289",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-05-31T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-05-31",
                "duration": 160,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 295098129,
                "title": "Watcher",
                "cover": "ec69a060-1135-4458-b4e2-cedf1cca445a",
                "vibrantColor": "#43a78b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/295098129",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-05-19T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-05-19",
                "duration": 176,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 327895947,
                "title": "United We Ball (feat. XELLY & chemtrails)",
                "cover": "79d763ae-8673-40b8-a498-bea3be6c9ed1",
                "vibrantColor": "#ec6354",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/327895947",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-05-05T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-05-05",
                "duration": 198,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 291585873,
                "title": "Carillon",
                "cover": "ccec88f0-3c66-490b-9b27-6ec9195b5b30",
                "vibrantColor": "#f4670b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/291585873",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-05-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-05-01",
                "duration": 132,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 284244498,
                "title": "UP",
                "cover": "cfc2c685-d568-4bf0-b888-83552ab2c5bf",
                "vibrantColor": "#ce4d39",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/284244498",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-04-14T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-04-14",
                "duration": 164,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 284318374,
                "title": "Sidestep EP",
                "cover": "32c21bbe-c23f-4680-9129-44e51f1f6c1c",
                "vibrantColor": "#1aa5de",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/284318374",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-04-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 2,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-04-07",
                "duration": 747,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 327895945,
                "title": "Class (feat. Jesus Alfaro)",
                "cover": "5dd7dd06-6642-4f79-a306-8ff9d4a92db8",
                "vibrantColor": "#ec6354",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/327895945",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-03-17T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-03-17",
                "duration": 123,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 272957360,
                "title": "Michael J.",
                "cover": "34b6abdc-dd26-45c0-96c2-9517da437f17",
                "vibrantColor": "#e598ae",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/272957360",
                "artists": [
                  {
                    "id": 29410381,
                    "name": "Tomsn98",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-02-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-02-10",
                "duration": 129,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 271776480,
                "title": "Cuggi",
                "cover": "2f67bb02-2647-4bd2-807b-497377980a6d",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/271776480",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 35097970,
                    "name": "Sono Seba",
                    "type": "FEATURED",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2023-01-30T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-01-30",
                "duration": 123,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 260668105,
                "title": "Piece Of You",
                "cover": "9a9e24cc-7713-4c73-a9b1-0e71a56938cb",
                "vibrantColor": "#e66b6e",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/260668105",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-11-18T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-11-18",
                "duration": 202,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 255975105,
                "title": "Paranoia",
                "cover": "1be717cb-b415-4f7d-916d-ffc97c2e7eb7",
                "vibrantColor": "#e8b95d",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/255975105",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2022-11-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-11-11",
                "duration": 153,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 257890138,
                "title": "Perdón",
                "cover": "42784852-cdf7-4f34-8c56-278b333326ce",
                "vibrantColor": "#c48bdc",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/257890138",
                "artists": [
                  {
                    "id": 9009454,
                    "name": "Ticli",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-11-04T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-11-04",
                "duration": 168,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 257694030,
                "title": "No estoy",
                "cover": "5a2b1007-e011-450b-98d0-cdcb64e5a654",
                "vibrantColor": "#f6cf8c",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/257694030",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-10-27T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-10-27",
                "duration": 178,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 366598934,
                "title": "A$AP ROCKY",
                "cover": "94a3ce0c-c5a6-4544-8e07-ac1a48470893",
                "vibrantColor": "#5781e0",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/366598934",
                "artists": [
                  {
                    "id": 19468372,
                    "name": "Jonnas Rosa",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 15180774,
                    "name": "Brrioni",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2024-06-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-10-25",
                "duration": 166,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 252151291,
                "title": "Tanah Ini",
                "cover": "500ada7f-0734-4680-8b01-d4cc4bade176",
                "vibrantColor": "#eac775",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/252151291",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-10-06T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-10-06",
                "duration": 300,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 246108024,
                "title": "Pelis, tragos y canciones",
                "cover": "b9da3490-a3b3-4521-8285-537626a9d1cb",
                "vibrantColor": "#f2d389",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/246108024",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-09-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-09-01",
                "duration": 197,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 255290307,
                "title": "暗恋",
                "cover": "beddaf9b-7f64-42d5-82ee-6aaf44d5458e",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/255290307",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-09-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-09-01",
                "duration": 185,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 238518689,
                "title": "Flak",
                "cover": "16ee70b0-f083-409e-8fd1-e66b3d8dc7bc",
                "vibrantColor": "#9ab9e2",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/238518689",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-07-18T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-07-18",
                "duration": 182,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 236901261,
                "title": "Lets Go",
                "cover": "81be719f-d26b-4fdb-af46-f83895266de4",
                "vibrantColor": "#6b69d0",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/236901261",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2022-06-28T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-06-28",
                "duration": 160,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 233896514,
                "title": "A donde voy",
                "cover": "657be5cd-0b7c-47ad-a06d-8ebba83c3b7f",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/233896514",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-06-16T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-06-16",
                "duration": 178,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 230167428,
                "title": "Bleach Wid It (feat. Vesion)",
                "cover": "c1a06c6a-4866-4a85-8676-e267897dbbf6",
                "vibrantColor": "#854038",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/230167428",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2022-05-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-05-20",
                "duration": 184,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 272062785,
                "title": "That's Life",
                "cover": "c4084988-67ef-485b-9c4a-4403eeeb93d8",
                "vibrantColor": "#5ca2d7",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/272062785",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-05-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-05-20",
                "duration": 155,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 303724819,
                "title": "峡岭",
                "cover": "98260850-116f-4213-9d7c-3d2ad5ba0b80",
                "vibrantColor": "#addcea",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/303724819",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-07-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 4,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-03-24",
                "duration": 666,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 218277096,
                "title": "Pra Todos os Efeitos",
                "cover": "b109ce88-2d4c-41c6-887a-5cf53c46481e",
                "vibrantColor": "#db5c8b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/218277096",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-03-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-03-11",
                "duration": 223,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 219709456,
                "title": "Toast To Life",
                "cover": "29c3c34e-801b-4b8e-a2bf-c4ca535eba77",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/219709456",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-03-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-03-07",
                "duration": 170,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 213555872,
                "title": "Ulisse",
                "cover": "fca7ab75-c348-4459-b372-0d245fe73099",
                "vibrantColor": "#4861b7",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/213555872",
                "artists": [
                  {
                    "id": 4279312,
                    "name": "Gas",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2022-01-24T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 1,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-01-24",
                "duration": 82,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              }
            ],
            "dataApiPath": "pages/data/bb502cc2-58f7-4bd1-870a-265658fa36af?artistId=4279312"
          },
          "header": null,
          "layout": null
        }
      ]
    },
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiI0OWY1MjQ1Zi1jMDNlLTRmNDQtOTgxMS1hYzRlMGYyNDMwMmMiLCJtViI6MSwibUgiOiJkODZkYWFjMSJ9",
          "type": "ALBUM_LIST",
          "width": 100,
          "title": "Appears On",
          "description": "",
          "preTitle": null,
          "showMore": {
            "title": "View all",
            "apiPath": "pages/single-module-page/ae223310-a4c2-4568-a770-ffef70344441/4/49f5245f-c03e-4f44-9811-ac4e0f24302c/1?artistId=4279312"
          },
          "supportsPaging": true,
          "quickPlay": false,
          "scroll": "HORIZONTAL",
          "listFormat": null,
          "pagedList": {
            "limit": 50,
            "offset": 0,
            "totalNumberOfItems": 162,
            "items": [
              {
                "id": 26521424,
                "title": "Sextape",
                "cover": "636fd2d4-5b78-4f8e-ae2f-f1fcf68b149b",
                "vibrantColor": "#fcded3",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/26521424",
                "artists": [
                  {
                    "id": 5165945,
                    "name": "Willie Taylor",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2014-02-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 16,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2010-01-01",
                "duration": 3621,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 100737032,
                "title": "Classic Pimpin' (live)",
                "cover": "d107dcf4-fbb3-48cc-a264-deed12408856",
                "vibrantColor": "#f0ba54",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/100737032",
                "artists": [
                  {
                    "id": 19094,
                    "name": "8Ball & MJG",
                    "type": "MAIN",
                    "picture": "08da1aa9-4486-455e-9ad5-7f4e317c74ef"
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2019-01-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-01-01",
                "duration": 3059,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 7420309,
                "title": "The Dayton Family Presents: The Right to Remain Silent EP",
                "cover": "f78c6f84-cdb6-4b0a-bcda-723980e18bf8",
                "vibrantColor": "#efe069",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/7420309",
                "artists": [
                  {
                    "id": 4054229,
                    "name": "Shoestring",
                    "type": "MAIN",
                    "picture": "18dfbdc3-82a1-4436-b0d4-852b090c981b"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2011-06-21T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 7,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2011-06-21",
                "duration": 1422,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 74171526,
                "title": "Blueberry Café, Vol. 1 (Jazzy & House Moods)",
                "cover": "28f85694-3566-43c2-b19c-354a258c6033",
                "vibrantColor": "#debee8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/74171526",
                "artists": [
                  {
                    "id": 3983743,
                    "name": "Marga Sol",
                    "type": "MAIN",
                    "picture": "27cd6b89-cd33-47ba-b3e3-2b8e002e3280"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2013-12-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 16,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2013-12-07",
                "duration": 5180,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 194254157,
                "title": "Connecting The Dots (DJ Mix)",
                "cover": "0c8b9d72-e1ce-4184-9288-160d88ad4c45",
                "vibrantColor": "#f96a1b",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/194254157",
                "artists": [
                  {
                    "id": 7358739,
                    "name": "Alex Paterson",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 19435,
                    "name": "The Orb",
                    "type": "MAIN",
                    "picture": "7ffcc075-8cd4-4c46-b4c9-353d35ec829f"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2021-09-17T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 17,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2021-09-17",
                "duration": 5482,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 113357628,
                "title": "Summer Vibes 2019",
                "cover": "697492d8-bd8a-4a7c-abed-ec332af3e91a",
                "vibrantColor": "#cde4fa",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/113357628",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-07-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 20,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-07-12",
                "duration": 3881,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 79439059,
                "title": "Zumba & Fitness 2018 - Latin Hits And Reggaeton From 100 To 128 BPM For Gym And Dance",
                "cover": "8c061f34-2672-4aed-b6d1-0bc02ac3e2ad",
                "vibrantColor": "#d7dd7f",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/79439059",
                "artists": [
                  {
                    "id": 6329968,
                    "name": "Paul Mice",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4530352,
                    "name": "Eric Russo",
                    "type": "MAIN",
                    "picture": null
                  },
                  {
                    "id": 4545004,
                    "name": "David Ferrari",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2017-10-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 18,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2017-10-01",
                "duration": 4073,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 29055628,
                "title": "Tha Eastside Mayor",
                "cover": "bc35d1b4-8bbf-4cf3-b40c-1710c2e73b38",
                "vibrantColor": "#d99695",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/29055628",
                "artists": [
                  {
                    "id": 5590023,
                    "name": "Forty Da Great",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2013-01-22T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 16,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2013-01-22",
                "duration": 3580,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 331342018,
                "title": "Connecting The Dots (DJ Mix)",
                "cover": "fdb35d2b-1ad9-4f56-b999-1130a4d7c14b",
                "vibrantColor": "#eee1f1",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/331342018",
                "artists": [
                  {
                    "id": 9585,
                    "name": "Miss Kittin",
                    "type": "MAIN",
                    "picture": "abe35180-21c1-483e-bc70-2711e310b98b"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-12-15T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 16,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-12-15",
                "duration": 4164,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 105043712,
                "title": "Aerobic Dance Latino 2019 - 1 Hour Non Stop Music Mix For Aerobics, Step & Workout",
                "cover": "ec6cac94-9628-4a8b-9b3f-6076ca22d6a9",
                "vibrantColor": "#ead06e",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/105043712",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-03-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 19,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-03-01",
                "duration": 9147,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 154865522,
                "title": "Pop Ambient 2005",
                "cover": "c2480128-5ca4-4819-bcb1-a2e0e70ada4a",
                "vibrantColor": "#d699a1",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/154865522",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2005-02-21T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2004-11-29",
                "duration": 4090,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 163605677,
                "title": "X Mas Dance 2020",
                "cover": "cc4c0528-214a-4c49-acb8-6b163aeb9e61",
                "vibrantColor": "#ead387",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/163605677",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-12-11T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-12-11",
                "duration": 3084,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 67138058,
                "title": "EDM For Running & Workout",
                "cover": "2a6c3737-cdd0-4514-90c0-ce3399b8cd07",
                "vibrantColor": "#f8eda2",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/67138058",
                "artists": [
                  {
                    "id": 6978558,
                    "name": "David Pole",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-11-02T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 19,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-11-02",
                "duration": 4574,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 57268942,
                "title": "Addicted To Latin",
                "cover": "d6a711da-1388-43e6-95e4-c7e2538a0ab4",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/57268942",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-02-19T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-02-11",
                "duration": 4299,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 81435206,
                "title": "Spinning 2018 - Energy & Power. Music For Spinning & Indoor Bike",
                "cover": "c30314cc-d3ba-442b-852c-a08a6b316c21",
                "vibrantColor": "#a2bdd9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/81435206",
                "artists": [
                  {
                    "id": 4981792,
                    "name": "MJX",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2017-11-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 18,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2017-11-23",
                "duration": 4742,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 26998058,
                "title": "Goa Classics, Vol. 7",
                "cover": "baff74cb-4243-44c2-ba99-308ceda90a71",
                "vibrantColor": "#f5de89",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/26998058",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2014-03-09T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 33,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2014-03-09",
                "duration": 14528,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 106468254,
                "title": "Milkshake, Vol. 1",
                "cover": "d8d9d4fe-1573-4569-800e-8599867221e8",
                "vibrantColor": "#e0b1b0",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/106468254",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-03-29T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-03-29",
                "duration": 2520,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 267081802,
                "title": "Winter Hits Dance 2022 - Deep, House, Tropical, Edm, Pop, Dance, Latin Music Hits",
                "cover": "28eba7a6-f2d8-4c44-82fe-ecaa17ebd1ad",
                "vibrantColor": "#52e8e9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/267081802",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-12-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 24,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-12-01",
                "duration": 4312,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 271888632,
                "title": "Bleep:10",
                "cover": "ccc96278-bd04-4787-9b5a-4eb49aa0ec84",
                "vibrantColor": "#b0ffbe",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/271888632",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2014-05-05T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 14,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2014-05-05",
                "duration": 4067,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 27001268,
                "title": "Goa Beasts, Vol. 25",
                "cover": "9bd2d327-2f3a-4901-983a-c92b1dc2e195",
                "vibrantColor": "#fbd993",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/27001268",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2014-05-17T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 24,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2014-03-09",
                "duration": 11055,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 246433864,
                "title": "Mojito Vocal Deep House (Aperitive Session)",
                "cover": "270c7bdc-1cb3-47ab-b1df-822497ed7a97",
                "vibrantColor": "#aa2230",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/246433864",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-09-09T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 16,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-09-09",
                "duration": 3343,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 241415601,
                "title": "Coffee House (Toasted Music Selection Vocal Deep House)",
                "cover": "cd73fd23-e08e-4ce8-849d-29500920bbdd",
                "vibrantColor": "#a0d9d8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/241415601",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-08-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 14,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-08-12",
                "duration": 2980,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 249074521,
                "title": "Sunday Chill (Deep House)",
                "cover": "0df96411-0c27-444a-92e3-cb898f78f4bc",
                "vibrantColor": "#f7c368",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/249074521",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-09-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 14,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-09-23",
                "duration": 2844,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 249074468,
                "title": "Formentera Deep House",
                "cover": "51823caa-43b5-417c-bc57-3a5c83c684d7",
                "vibrantColor": "#decb88",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/249074468",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-09-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 14,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-09-23",
                "duration": 2833,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 241408079,
                "title": "Beverly Hills Restaurants",
                "cover": "e752948b-b2cd-4d73-a95a-fefb39d57cca",
                "vibrantColor": "#9bc4e9",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/241408079",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-08-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-08-12",
                "duration": 2463,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS",
                    "HIRES_LOSSLESS"
                  ]
                }
              },
              {
                "id": 59880475,
                "title": "Playa Latina 2016",
                "cover": "09c02c78-1963-487c-81c5-ccf9de52fccd",
                "vibrantColor": "#9ee4d6",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/59880475",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-04-22T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-04-22",
                "duration": 3090,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 9703190,
                "title": "Somerson",
                "cover": "04736d9a-24bf-4638-8f13-dfd1cfe2e5ee",
                "vibrantColor": "#79a2e8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/9703190",
                "artists": [
                  {
                    "id": 4243721,
                    "name": "Gas!",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2008-12-05T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 12,
                "numberOfVideos": 0,
                "audioQuality": "HIGH",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2008-12-05",
                "duration": 2622,
                "mediaMetadata": {
                  "tags": []
                }
              },
              {
                "id": 82189284,
                "title": "Workout 2018 - Aerobic Hits. Music For Fitness & Workout 128 Bpm / 32 Count",
                "cover": "169f58bf-d150-481e-ab66-d1883e4875c0",
                "vibrantColor": "#a0b9d7",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/82189284",
                "artists": [
                  {
                    "id": 5197341,
                    "name": "Marcus Lanzer",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2017-12-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 18,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2017-12-20",
                "duration": 5030,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 58218532,
                "title": "Best of Miami Latin Session",
                "cover": "305ac66e-a3b8-428c-a261-b3bd7934d16f",
                "vibrantColor": "#ded2a5",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/58218532",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-03-18T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-03-18",
                "duration": 4202,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 56556690,
                "title": "Pure Latin Vibes",
                "cover": "f7aae3eb-fa4c-4ee4-8c49-c13ea547e929",
                "vibrantColor": "#f8a198",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/56556690",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2016-01-29T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2016-01-29",
                "duration": 3728,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 263018486,
                "title": "Viva Latino 2023 - Latin Hit Mix - Latin Pop, Reggaeton, Latin Urban Top Hits",
                "cover": "3a7f8e58-6fc9-4e1a-a0e1-2a1973d1a81d",
                "vibrantColor": "#ed3f99",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/263018486",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2022-12-06T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 24,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2022-12-06",
                "duration": 4589,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 28637650,
                "title": "Piment Rouge 2",
                "cover": "025c0b46-7654-4b0f-a9ce-e45fa98b5eaa",
                "vibrantColor": "#f6d894",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/28637650",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2014-04-15T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 19,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2008-01-01",
                "duration": 3619,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 113895173,
                "title": "Inland Versions",
                "cover": "6335b105-34e5-4ee6-bc5c-fce8efeb300a",
                "vibrantColor": "#FFFFFF",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/113895173",
                "artists": [
                  {
                    "id": 4342960,
                    "name": "Vanessa Wagner",
                    "type": "MAIN",
                    "picture": "ffad76f1-2606-47b2-95e5-b8c5a77c67e2"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-09-27T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 6,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-09-27",
                "duration": 2180,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 154864932,
                "title": "Pop Ambient 2007",
                "cover": "41144016-81d4-4e99-ad0f-0cd92cc7c77c",
                "vibrantColor": "#f6cd67",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/154864932",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2006-11-20T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 10,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2006-12-07",
                "duration": 3682,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 120934175,
                "title": "Mykonos Aqua Gym Latin Hits Workout Collection (15 Tracks Non-Stop Mixed Compilation for Fitness & Workout - 128 Bpm / 32 Count)",
                "cover": "26c52fb1-f0b9-4313-b4d4-b58a4a9d2b43",
                "vibrantColor": "#cadf54",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/120934175",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-11-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-11-01",
                "duration": 3441,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 26451707,
                "title": "We Drop Tracks! (A Deep House Selection)",
                "cover": "1bb3141c-cc22-42e8-b64f-f2f6c15d6f15",
                "vibrantColor": "#f6a247",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/26451707",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2014-03-03T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 26,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2014-02-18",
                "duration": 15841,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 123259544,
                "title": "Latin Remixes 2019",
                "cover": "54e43ed3-9ec0-413b-bf4b-3edd18704858",
                "vibrantColor": "#9febd8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/123259544",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-11-29T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 20,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-11-29",
                "duration": 4795,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 304949956,
                "title": "Summer Club Hits 2023",
                "cover": "2b157ef5-081f-4398-aff3-74b4c1d16d80",
                "vibrantColor": "#e8d893",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/304949956",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-07-19T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 52,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-07-19",
                "duration": 11752,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 118315213,
                "title": "Sabor Latino - 30 Top Latin House Tracks",
                "cover": "6bc1a5fe-37f8-4033-9536-9c2c72927e11",
                "vibrantColor": "#e2d79f",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/118315213",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2015-01-01T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 30,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2015-01-01",
                "duration": 8488,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 86427210,
                "title": "Energy of Flat Abs Latin Hits Session",
                "cover": "e402e098-c603-48bb-a084-bcc966aecb79",
                "vibrantColor": "#bca371",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/86427210",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2018-03-30T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2018-03-30",
                "duration": 3758,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 75357270,
                "title": "Aqua Gym 2017 - Music For Aquagym, Aqua Biking, Aqua Fitness.",
                "cover": "d0e43cc8-7707-4f5e-b65f-e7c1ccb91567",
                "vibrantColor": "#b8eff4",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/75357270",
                "artists": [
                  {
                    "id": 8275394,
                    "name": "J Clement",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2017-06-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 18,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2017-06-23",
                "duration": 5265,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 17511880,
                "title": "Expansion",
                "cover": "c003a90c-c1e8-4016-9bbe-5090471abcfc",
                "vibrantColor": "#f4eed5",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/17511880",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2012-10-19T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 10,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2012-10-19",
                "duration": 4525,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 134895454,
                "title": "Loud & Dirty - The Electro House Collection, Vol. 32",
                "cover": "cc1eaea5-13d2-4837-ae79-4832d661efba",
                "vibrantColor": "#e0a2e6",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/134895454",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": true,
                "streamReady": true,
                "streamStartDate": "2020-04-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 20,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-04-10",
                "duration": 5225,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 48352180,
                "title": "Playa Latina Summer 2015",
                "cover": "bec2ad94-e9e5-47b5-bbbf-7fefa9ff2b31",
                "vibrantColor": "#e961c1",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/48352180",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2015-06-26T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 20,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2015-06-26",
                "duration": 5844,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 84908815,
                "title": "Tokyo Street Workout Edm Workout Compilation",
                "cover": "06e13cd6-0351-4ea4-b5c4-328f23e6ccb3",
                "vibrantColor": "#5278b8",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/84908815",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2018-02-23T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2018-02-23",
                "duration": 3901,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 106023374,
                "title": "Spring Aqua Gym Hits 2019",
                "cover": "abc1b197-ab55-424d-9010-8721de98e2f7",
                "vibrantColor": "#d7ccab",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/106023374",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2019-03-22T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 15,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2019-03-22",
                "duration": 4029,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 296585786,
                "title": "Dub Techno District, Vol. 11",
                "cover": "b33f471c-fbfe-4dce-a487-ce88796d51b5",
                "vibrantColor": "#e9cb92",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/296585786",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2018-12-07T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 23,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2018-12-07",
                "duration": 9804,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 136773523,
                "title": "Aerobic Dance Latino 2020 - Latin Hits For Aerobics, Pump, Step & Workout",
                "cover": "b1021702-fb55-4167-a35b-9675e82c5b64",
                "vibrantColor": "#773583",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/136773523",
                "artists": [
                  {
                    "id": 4519945,
                    "name": "Danilo Orsini",
                    "type": "MAIN",
                    "picture": "a6a955ff-a31e-4a68-9144-4c03f18a0cc9"
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2020-04-10T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 20,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2020-04-10",
                "duration": 4172,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 269548592,
                "title": "Zumba Mania 2023 - Latin Electro House Hits for Fitness & Dance",
                "cover": "a58d88fc-c1f1-4d47-9675-a4eed5bb699f",
                "vibrantColor": "#f48961",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/269548592",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2023-01-12T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 18,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2023-01-12",
                "duration": 3312,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              },
              {
                "id": 73789812,
                "title": "Club Pressure - The Progressive and Clubsound Collection, Vol. 22",
                "cover": "61585a71-4879-4cff-aec3-789eac051e82",
                "vibrantColor": "#da9fed",
                "videoCover": null,
                "url": "https://tidal.com/browse/album/73789812",
                "artists": [
                  {
                    "id": 2935,
                    "name": "Various Artists",
                    "type": "MAIN",
                    "picture": null
                  }
                ],
                "explicit": false,
                "streamReady": true,
                "streamStartDate": "2017-05-24T00:00:00.000+00:00",
                "allowStreaming": true,
                "numberOfTracks": 17,
                "numberOfVideos": 0,
                "audioQuality": "LOSSLESS",
                "audioModes": [
                  "STEREO"
                ],
                "releaseDate": "2017-05-24",
                "duration": 4511,
                "mediaMetadata": {
                  "tags": [
                    "LOSSLESS"
                  ]
                }
              }
            ],
            "dataApiPath": "pages/data/5667e093-4bce-4f14-9292-bfdc20c9c3fe?artistId=4279312"
          },
          "header": null,
          "layout": null
        }
      ]
    },
    {
      "modules": [
        {
          "id": "eyJwIjoiYWUyMjMzMTAtYTRjMi00NTY4LWE3NzAtZmZlZjcwMzQ0NDQxIiwicFYiOjQsIm0iOiJiNGI5NTc5NS03NzhiLTQ5YzUtYTM0Zi01OWFhYzA1NWI2NjIiLCJtViI6MSwibUgiOiIxMmU2MGRmNCJ9",
          "type": "ARTIST_LIST",
          "width": 100,
          "title": "Fans Also Like",
          "description": "",
          "preTitle": null,
          "showMore": {
            "title": "View all",
            "apiPath": "pages/single-module-page/ae223310-a4c2-4568-a770-ffef70344441/4/b4b95795-778b-49c5-a34f-59aac055b662/1?artistId=4279312"
          },
          "supportsPaging": true,
          "quickPlay": false,
          "scroll": "HORIZONTAL",
          "pagedList": {
            "limit": 15,
            "offset": 0,
            "totalNumberOfItems": 15,
            "items": [
              {
                "id": 7311578,
                "name": "Nevermen",
                "picture": "ac0b1884-3754-4390-8c99-633e6af6ffa3",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "000a8b74ca2c4b6e76b58b9a12058c"
                }
              },
              {
                "id": 3575760,
                "name": "The Black Dog",
                "picture": "840b9e68-5fe3-4157-8fe4-3d6d24cb5880",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0004d3d74e00face0268b364aebbc2"
                }
              },
              {
                "id": 4132466,
                "name": "Kettel",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Performer",
                    "categoryId": 11
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "000bfa6e9d47832d358f177c59f7ce"
                }
              },
              {
                "id": 3528833,
                "name": "Proem",
                "picture": "977f5ca4-287a-4b4d-96ba-136662fe7dba",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0009deeda005f906f5294f8ee7bf5c"
                }
              },
              {
                "id": 3549352,
                "name": "Frank Bretschneider",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0004a9903029834694fede06bf4438"
                }
              },
              {
                "id": 3521085,
                "name": "Lusine Icl",
                "picture": "35936138-8c20-4b28-a58a-9cc12a0a6162",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "000326a8feb8d9bfbbb5d379843136"
                }
              },
              {
                "id": 4439455,
                "name": "Leandro Fresco",
                "picture": "018d9eed-c9b3-4858-a475-d88bcabcc8c1",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Performer",
                    "categoryId": 11
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0005e17c77e001240eaad47977ff28"
                }
              },
              {
                "id": 3642009,
                "name": "The Sight Below",
                "picture": "925cc3c0-f01e-4e8d-a114-557c476b7dca",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "000bcdd2659af796ab3ca14ad1b87a"
                }
              },
              {
                "id": 5345342,
                "name": "Ulf Lohmann",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "000817bb3b0c8b0e9b125b1aa42f9e"
                }
              },
              {
                "id": 3693340,
                "name": "Blamstrain",
                "picture": "4c91443d-bcca-45c3-ad40-a0e597a2ab03",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "00036b11bf871df5293183ef6dae3d"
                }
              },
              {
                "id": 3551834,
                "name": "Pjusk",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0007318db6f136d60891210a014a0d"
                }
              },
              {
                "id": 5678887,
                "name": "Ocoeur",
                "picture": "2abf33f7-689f-4e72-8667-7e6e8afd24dd",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "00047191a786613790a809d86239c8"
                }
              },
              {
                "id": 4550621,
                "name": "Miwon",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Performer",
                    "categoryId": 11
                  },
                  {
                    "category": "Production team",
                    "categoryId": 10
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  },
                  {
                    "category": "Producer",
                    "categoryId": 1
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0007b1929b276a9f1b53314ba88b45"
                }
              },
              {
                "id": 3949096,
                "name": "Subheim",
                "picture": null,
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  },
                  {
                    "category": "Performer",
                    "categoryId": 11
                  },
                  {
                    "category": "Songwriter",
                    "categoryId": 2
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "0006f077d8a72d8bbf33c88d1de147"
                }
              },
              {
                "id": 3854272,
                "name": "Between Interval",
                "picture": "b6dfed22-c846-4035-add2-c8311d87c21c",
                "artistTypes": null,
                "artistRoles": [
                  {
                    "category": "Artist",
                    "categoryId": -1
                  },
                  {
                    "category": "Engineer",
                    "categoryId": 3
                  }
                ],
                "mixes": {
                  "ARTIST_MIX": "00022441a7497f154e927927f8ee6d"
                }
              }
            ],
            "dataApiPath": "pages/data/5c8d0b08-b48d-4a97-bcbd-989c81f0c96e?artistId=4279312"
          },
          "header": null,
          "layout": null
        }
      ]
    }
  ]
}"##;
