# comics-rs
cli utility for comics - currently comicvine only

## Currently supported comands
### Note: These commands automatically select the first result
`-s` Search for a series
Example:
#### Search for the series "Black Lagoon"
``` shell
  comics -s 'Black Lagoon'
```
<details><summary>Output</summary>
<p>

```
Results {
    aliases: Null,
    api_detail_url: Some(
        "https://comicvine.gamespot.com/api/series/4075-53/",
    ),
    characters: Some(
        [
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-72334/",
                ),
                id: 72334,
                name: Some(
                    "Revy",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/revy/4005-72334/",
                ),
                count: Some(
                    "12",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88796/",
                ),
                id: 88796,
                name: Some(
                    "Rock",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/rock/4005-88796/",
                ),
                count: Some(
                    "12",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88797/",
                ),
                id: 88797,
                name: Some(
                    "Dutch",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/dutch/4005-88797/",
                ),
                count: Some(
                    "11",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88798/",
                ),
                id: 88798,
                name: Some(
                    "Benny",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/benny/4005-88798/",
                ),
                count: Some(
                    "11",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-86631/",
                ),
                id: 86631,
                name: Some(
                    "Balalaika",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/balalaika/4005-86631/",
                ),
                count: Some(
                    "7",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-86626/",
                ),
                id: 86626,
                name: Some(
                    "Roberta",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/roberta/4005-86626/",
                ),
                count: Some(
                    "3",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88801/",
                ),
                id: 88801,
                name: Some(
                    "Shenhua",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/shenhua/4005-88801/",
                ),
                count: Some(
                    "2",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88856/",
                ),
                id: 88856,
                name: Some(
                    "Mr. Chang",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/mr-chang/4005-88856/",
                ),
                count: Some(
                    "2",
                ),
            },
            Character {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/character/4005-88800/",
                ),
                id: 88800,
                name: Some(
                    "Eda",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/eda/4005-88800/",
                ),
                count: Some(
                    "1",
                ),
            },
        ],
    ),
    count_of_episodes: 12,
    date_added: Some(
        "2013-09-25 13:36:02",
    ),
    date_last_updated: Some(
        "2013-09-25 14:14:04",
    ),
    deck: String(
        "Black Lagoon is the anime adaptation of the seinen manga series from Rei Hiroe.",
    ),
    description: Some(
        "<p data-right-indent=\"0\" data-left-indent=\"0\"><em>Black Lagoon</em> (ブラック・ラグーン, <em>Burakku Ragūn</em>)</p><p data-right-indent=\"0\" data-left-indent=\"0\">Continued in <a href=\"//www.comicvine.com/black-lagoon-the-second-barrage/4075-54/\" data-ref-id=\"4075-54\"><em>Black Lagoon: The Second Barrage</em></a></p>",
    ),
    episodes: Some(
        [
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-592/",
                ),
                id: 592,
                name: Some(
                    "The Black Lagoon",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-101-the-black-lagoon/4070-592/",
                ),
                episode_number: Some(
                    "101",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1086/",
                ),
                id: 1086,
                name: Some(
                    "Mangrove Heaven",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-102-mangrove-heaven/4070-1086/",
                ),
                episode_number: Some(
                    "102",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1087/",
                ),
                id: 1087,
                name: Some(
                    "Ring-Ding Ship Chase",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-103-ring-ding-ship-chase/4070-1087/",
                ),
                episode_number: Some(
                    "103",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1088/",
                ),
                id: 1088,
                name: Some(
                    "Die Rückkehr des Adlers",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-104-die-ruckkehr-des-adlers/4070-1088/",
                ),
                episode_number: Some(
                    "104",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1092/",
                ),
                id: 1092,
                name: Some(
                    "Eagle Hunting and Hunting Eagles",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-105-eagle-hunting-and-hunting-eagles/4070-1092/",
                ),
                episode_number: Some(
                    "105",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1093/",
                ),
                id: 1093,
                name: Some(
                    "Moonlit Hunting Grounds",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-106-moonlit-hunting-grounds/4070-1093/",
                ),
                episode_number: Some(
                    "106",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1095/",
                ),
                id: 1095,
                name: Some(
                    "Calm Down, Two Men",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-107-calm-down-two-men/4070-1095/",
                ),
                episode_number: Some(
                    "107",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1096/",
                ),
                id: 1096,
                name: Some(
                    "Rasta Blasta",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-108-rasta-blasta/4070-1096/",
                ),
                episode_number: Some(
                    "108",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1098/",
                ),
                id: 1098,
                name: Some(
                    "Maid to Kill",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-109-maid-to-kill/4070-1098/",
                ),
                episode_number: Some(
                    "109",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1099/",
                ),
                id: 1099,
                name: Some(
                    "The Unstoppable Chambermaid",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-110-the-unstoppable-chambermaid/4070-1099/",
                ),
                episode_number: Some(
                    "110",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1100/",
                ),
                id: 1100,
                name: Some(
                    "Lock\'n Load Revolution",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-111-lockn-load-revolution/4070-1100/",
                ),
                episode_number: Some(
                    "111",
                ),
            },
            Episode {
                api_detail_url: Some(
                    "https://comicvine.gamespot.com/api/episode/4070-1101/",
                ),
                id: 1101,
                name: Some(
                    "Guerrillas in the Jungle",
                ),
                site_detail_url: Some(
                    "https://comicvine.gamespot.com/black-lagoon-112-guerrillas-in-the-jungle/4070-1101/",
                ),
                episode_number: Some(
                    "112",
                ),
            },
        ],
    ),
    first_episode: Some(
        FirstEpisode {
            api_detail_url: Some(
                "https://comicvine.gamespot.com/api/episode/4070-592/",
            ),
            id: 592,
            name: Some(
                "The Black Lagoon",
            ),
            episode_number: Some(
                "101",
            ),
        },
    ),
    id: 53,
    image: Image {
        icon_url: Some(
            "https://comicvine.gamespot.com/api/image/square_avatar/3334428-7646102968-2346-.jpg",
        ),
        medium_url: Some(
            "https://comicvine.gamespot.com/api/image/scale_medium/3334428-7646102968-2346-.jpg",
        ),
        screen_url: Some(
            "https://comicvine.gamespot.com/api/image/screen_medium/3334428-7646102968-2346-.jpg",
        ),
        screen_large_url: Some(
            "https://comicvine.gamespot.com/api/image/screen_kubrick/3334428-7646102968-2346-.jpg",
        ),
        small_url: Some(
            "https://comicvine.gamespot.com/api/image/scale_small/3334428-7646102968-2346-.jpg",
        ),
        super_url: Some(
            "https://comicvine.gamespot.com/api/image/scale_large/3334428-7646102968-2346-.jpg",
        ),
        thumb_url: Some(
            "https://comicvine.gamespot.com/api/image/scale_avatar/3334428-7646102968-2346-.jpg",
        ),
        tiny_url: Some(
            "https://comicvine.gamespot.com/api/image/square_mini/3334428-7646102968-2346-.jpg",
        ),
        original_url: Some(
            "https://comicvine.gamespot.com/api/image/original/3334428-7646102968-2346-.jpg",
        ),
        image_tags: Some(
            "All Images",
        ),
    },
    last_episode: Some(
        LastEpisode {
            api_detail_url: Some(
                "https://comicvine.gamespot.com/api/episode/4070-1101/",
            ),
            id: 1101,
            name: Some(
                "Guerrillas in the Jungle",
            ),
            episode_number: Some(
                "112",
            ),
        },
    ),
    name: Some(
        "Black Lagoon",
    ),
    publisher: Some(
        Publisher {
            api_detail_url: Some(
                "https://comicvine.gamespot.com/api/publisher/4010-2781/",
            ),
            id: 2781,
            name: Some(
                "Shogakukan",
            ),
        },
    ),
    site_detail_url: Some(
        "https://comicvine.gamespot.com/black-lagoon/4075-53/",
    ),
    start_year: Some(
        "2006",
    ),
}
```
</p>
</details>

___

`-a` Search for a story arc
Example:
#### Search for the story arc "Night of the Owls"
``` shell
comics -a 'Night of the Owls
```
<details><summary>Output</summary>
<p>

```
Issues:
Bloodlines - URL: https://comicvine.gamespot.com/nightwing-8-bloodlines/4000-331843/
Attack on Wayne Manor; The Call - URL: https://comicvine.gamespot.com/batman-8-attack-on-wayne-manor-the-call/4000-331842/
The Night of the Owls; The Fall of the House of Wayne, Part 1 of 3 - URL: https://comicvine.gamespot.com/batman-9-the-night-of-the-owls-the-fall-of-the-hou/4000-335017/
Robin Hears a Hoo - URL: https://comicvine.gamespot.com/batman-and-robin-9-robin-hears-a-hoo/4000-335018/
Vengeance in the Big Easy; The King of Carnival - URL: https://comicvine.gamespot.com/all-star-western-9-vengeance-in-the-big-easy-the-k/4000-336574/
I Can No Longer be Broken - URL: https://comicvine.gamespot.com/batman-the-dark-knight-9-i-can-no-longer-be-broken/4000-336573/
Gangland Style - URL: https://comicvine.gamespot.com/birds-of-prey-9-gangland-style/4000-335896/
In the Line of Fire - URL: https://comicvine.gamespot.com/batgirl-9-in-the-line-of-fire/4000-335019/
You Have Been Judged Unworthy - URL: https://comicvine.gamespot.com/batwing-9-you-have-been-judged-unworthy/4000-334193/
Mirrors Come In All Sizes - URL: https://comicvine.gamespot.com/catwoman-9-mirrors-come-in-all-sizes/4000-335898/
Who are You? -- Hoo Hoo? - URL: https://comicvine.gamespot.com/red-hood-and-the-outlaws-9-who-are-you-hoo-hoo/4000-335897/
First Snow - URL: https://comicvine.gamespot.com/batman-annual-1-first-snow/4000-337480/
The Gray Son - URL: https://comicvine.gamespot.com/nightwing-9-the-gray-son/4000-335895/
The Owls Take Arkham; 50/50 - URL: https://comicvine.gamespot.com/detective-comics-9-the-owls-take-arkham-5050/4000-334192/
```

</p>
</details>
