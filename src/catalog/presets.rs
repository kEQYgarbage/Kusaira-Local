//! Curated, rich, valid anime dataset for seasonal ongoings, recommendations, and classics.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetAnime {
    pub id: u64,
    pub title: &'static str,
    pub name: &'static str,
    pub poster: &'static str,
    pub score: &'static str,
    pub pts: u32,
    pub episodes: u32,
    pub description: &'static str,
    pub release: &'static str,
}

pub fn get_preset_ongoing() -> Vec<PresetAnime> {
    vec![
        PresetAnime {
            id: 54638,
            title: "Дандадан",
            name: "Dandadan",
            poster: "https://shikimori.io/system/animes/original/54638.jpg",
            score: "8.72",
            pts: 97,
            episodes: 12,
            description: "Момо Аясэ верит в призраков, а её одноклассник Окарун — в инопланетян. Чтобы доказать друг другу правоту, они отправляются в аномальные зоны и сталкиваются со сверхъестественным.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] DEEP / Dream Cast",
        },
        PresetAnime {
            id: 52299,
            title: "Поднятие уровня в одиночку",
            name: "Solo Leveling: Ore dake Level Up na Ken",
            poster: "https://shikimori.io/system/animes/original/52299.jpg",
            score: "8.28",
            pts: 95,
            episodes: 12,
            description: "Десять лет назад открылись Врата в другие миры. Слабейший охотник Е-ранга Сон Джин-у чудом выживает в смертельном подземелье и получает уникальную способность Системы расти в силе.",
            release: "[WEB-DL 1080p HEVC AAC] Studio Band",
        },
        PresetAnime {
            id: 52991,
            title: "Провожающая в последний путь Фрирен",
            name: "Sousou no Frieren",
            poster: "https://shikimori.io/system/animes/original/52991.jpg",
            score: "9.31",
            pts: 99,
            episodes: 28,
            description: "Эльфийская волшебница Фрирен и её соратники победили Короля демонов. Спустя годы после кончины товарищей-людей она отправляется в путешествие, чтобы понять природу человеческих чувств.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] AniLibria",
        },
        PresetAnime {
            id: 55888,
            title: "Реинкарнация безработного 2",
            name: "Mushoku Tensei II: Isekai Ittara Honki Dasu",
            poster: "https://shikimori.io/system/animes/original/55888.jpg",
            score: "8.54",
            pts: 96,
            episodes: 24,
            description: "Рудеус Грейрат поступает в Магическую академию Раноа, чтобы разгадать тайну телепортационного инцидента и исцелить душевные раны после расставания с Эрис.",
            release: "[BDRip 1080p HEVC FLAC] AniLibria / Dream Cast",
        },
        PresetAnime {
            id: 52588,
            title: "Кайдзю №8",
            name: "Kaiju No. 8",
            poster: "https://shikimori.io/system/animes/original/52588.jpg",
            score: "8.15",
            pts: 94,
            episodes: 12,
            description: "Кафка Хибино мечтает вступить в Силы обороны Японии и истреблять чудовищ-кайдзю. Неожиданный инцидент наделяет его способностью самому трансформироваться в могучего монстра.",
            release: "[BDRip 1080p HEVC AAC] Crunchyroll / Studio Band",
        },
        PresetAnime {
            id: 41467,
            title: "Блич: Тысячелетняя кровавая война",
            name: "Bleach: Sennen Kessen-hen",
            poster: "https://shikimori.io/system/animes/original/41467.jpg",
            score: "8.98",
            pts: 98,
            episodes: 26,
            description: "Император квинси Яхве пробуждается от тысячелетнего сна и объявляет тотальную войну Обществу Душ. Ичиго Куросаки встаёт на защиту обоих миров.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] AniDUB / Studio Band",
        },
        PresetAnime {
            id: 52701,
            title: "Подземелье вкусностей",
            name: "Dungeon Meshi",
            poster: "https://shikimori.io/system/animes/original/52701.jpg",
            score: "8.61",
            pts: 96,
            episodes: 24,
            description: "Рыцарь Лай Торден и его отряд спускаются на глубинные этажи подземелья, чтобы спасти сестру Фалин из чрева дракона, выживая за счет кулинарных изысков из поверженных монстров.",
            release: "[WEB-DL 1080p HEVC AAC] AniLibria / Flarrow Films",
        },
        PresetAnime {
            id: 54865,
            title: "Синяя тюрьма: Блю Лок 2",
            name: "Blue Lock vs. U-20 Japan",
            poster: "https://shikimori.io/system/animes/original/54865.jpg",
            score: "7.92",
            pts: 93,
            episodes: 14,
            description: "Эгоистичные нападающие проекта 'Синяя тюрьма' бросают вызов молодежной национальной сборной Японии U-20 в решающем матче за выживание проекта.",
            release: "[WEB-DL 1080p HEVC AAC] Studio Band",
        },
        PresetAnime {
            id: 55701,
            title: "Клинок, рассекающий демонов: Тренировка",
            name: "Kimetsu no Yaiba: Hashira Geiko-hen",
            poster: "https://shikimori.io/system/animes/original/55701.jpg",
            score: "8.32",
            pts: 97,
            episodes: 8,
            description: "Истребители демонов во главе со Столпами начинают масштабную подготовку к решающей битве против прародителя демонов Мудзана Кибуцудзи.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Studio Band",
        },
        PresetAnime {
            id: 54857,
            title: "Re:Zero 3",
            name: "Re:Zero kara Hajimeru Isekai Seikatsu 3rd Season",
            poster: "https://shikimori.io/system/animes/original/54857.jpg",
            score: "8.65",
            pts: 96,
            episodes: 16,
            description: "Субару Нацуки и лагерь Эмилии отправляются в водный город Пристелла по приглашению Анастасии, где сталкиваются с коварным нападением архиепископов Культа Ведьмы.",
            release: "[WEB-DL 1080p HEVC AAC] AniLibria",
        },
    ]
}

pub fn get_preset_similar() -> Vec<PresetAnime> {
    vec![
        PresetAnime {
            id: 33352,
            title: "Вайолет Эвергарден",
            name: "Violet Evergarden",
            poster: "https://shikimori.io/system/animes/original/33352.jpg",
            score: "8.68",
            pts: 98,
            episodes: 13,
            description: "Бывшее живое оружие Вайолет устраивается работать автозапоминающей куклой, чтобы через письма других людей понять смысл последних слов майора: 'Я люблю тебя'.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Reanimedia",
        },
        PresetAnime {
            id: 34599,
            title: "Созданный в Бездне",
            name: "Made in Abyss",
            poster: "https://shikimori.io/system/animes/original/34599.jpg",
            score: "8.65",
            pts: 97,
            episodes: 13,
            description: "Юная Рико и робот-мальчик Рэг спускаются на самое дно загадочной гигантской вертикальной Бездны, таящей смертельные проклятия и древние реликвии.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] AniLibria",
        },
        PresetAnime {
            id: 37521,
            title: "Сага о Винланде",
            name: "Vinland Saga",
            poster: "https://shikimori.io/system/animes/original/37521.jpg",
            score: "8.75",
            pts: 97,
            episodes: 24,
            description: "Торфинн, сын великого воина Торса, посвящает юность жажде мести наемнику Аскеладду, путешествуя в отряде викингов по истерзанной войной Англии XI века.",
            release: "[BDRip 1080p HEVC FLAC] Studio Band",
        },
        PresetAnime {
            id: 40748,
            title: "Магическая битва",
            name: "Jujutsu Kaisen",
            poster: "https://shikimori.io/system/animes/original/40748.jpg",
            score: "8.62",
            pts: 96,
            episodes: 24,
            description: "Юдзи Итадори проглатывает палец Короля проклятий Сукуны и попадает в Магический техникум Токио, вступая в войну против порождений человеческой тьмы.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Studio Band",
        },
        PresetAnime {
            id: 44511,
            title: "Человек-бензопила",
            name: "Chainsaw Man",
            poster: "https://shikimori.io/system/animes/original/44511.jpg",
            score: "8.48",
            pts: 96,
            episodes: 12,
            description: "Дэндзи объединяется с демоном-бензопилой Почитой и становится охотником на демонов Общественной безопасности под началом загадочной Макимы.",
            release: "[BDRip 1080p HEVC FLAC] Crunchyroll / Studio Band",
        },
        PresetAnime {
            id: 52034,
            title: "Звёздное дитя",
            name: "Oshi no Ko",
            poster: "https://shikimori.io/system/animes/original/52034.jpg",
            score: "8.63",
            pts: 96,
            episodes: 11,
            description: "Доктор Горо перерождается ребенком своей любимой айдол-певицы Ай Хосино и погружается в темные закулисные интриги японского шоу-бизнеса.",
            release: "[BDRip 1080p HEVC FLAC] AniLibria",
        },
        PresetAnime {
            id: 47917,
            title: "Одинокий рокер!",
            name: "Bocchi the Rock!",
            poster: "https://shikimori.io/system/animes/original/47917.jpg",
            score: "8.78",
            pts: 97,
            episodes: 12,
            description: "Сверхтревожная гитаристка Хитори Гото 'Боччи' мечтает играть в группе и находит свой шанс в составе независимого коллектива Kessoku Band.",
            release: "[BDRip 1080p HEVC FLAC] AniLibria",
        },
        PresetAnime {
            id: 37520,
            title: "Дороро",
            name: "Dororo",
            poster: "https://shikimori.io/system/animes/original/37520.jpg",
            score: "8.24",
            pts: 94,
            episodes: 24,
            description: "Хяккимару, чье тело было принесено в жертву демонам собственным отцом, странствует по феодальной Японии с воришкой Дороро, возвращая органы одного за другим.",
            release: "[BDRip 1080p HEVC AAC] AniDUB",
        },
    ]
}

pub fn get_preset_ranked() -> Vec<PresetAnime> {
    vec![
        PresetAnime {
            id: 5114,
            title: "Стальной алхимик: Братство",
            name: "Fullmetal Alchemist: Brotherhood",
            poster: "https://shikimori.io/system/animes/original/5114.jpg",
            score: "9.10",
            pts: 99,
            episodes: 64,
            description: "Братья Эдвард и Альфонс Элрики нарушили главный запрет алхимии при попытке воскресить мать. Теперь они ищут Философский камень, чтобы вернуть утраченные тела.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Reanimedia",
        },
        PresetAnime {
            id: 9253,
            title: "Врата Штейна",
            name: "Steins;Gate",
            poster: "https://shikimori.io/system/animes/original/9253.jpg",
            score: "9.07",
            pts: 99,
            episodes: 24,
            description: "Самопровозглашенный безумный ученый Ринтаро Окабэ случайно создает устройство для отправки сообщений в прошлое, запуская цепь необратимых трагедий во временных линиях.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] AniDUB / Reanimedia",
        },
        PresetAnime {
            id: 11061,
            title: "Охотник х Охотник (2011)",
            name: "Hunter x Hunter (2011)",
            poster: "https://shikimori.io/system/animes/original/11061.jpg",
            score: "9.04",
            pts: 98,
            episodes: 148,
            description: "Гон Фрикс отправляется на опасный Экзамен на Охотника, чтобы найти своего отца Джина и познать необъятный мир диких зверей, сокровищ и секретов Нэн.",
            release: "[BDRip 1080p HEVC FLAC] AniDUB",
        },
        PresetAnime {
            id: 1535,
            title: "Тетрадь смерти",
            name: "Death Note",
            poster: "https://shikimori.io/system/animes/original/1535.jpg",
            score: "8.62",
            pts: 96,
            episodes: 37,
            description: "Гениальный старшеклассник Лайт Ягами находит тетрадь синигами и решает искоренить преступность, вступая в интеллектуальную дуэль с детективом L.",
            release: "[BDRip 1080p HEVC AAC] 2x2 / AniDUB",
        },
        PresetAnime {
            id: 1575,
            title: "Код Гиас: Восставший Лелуш",
            name: "Code Geass: Hangyaku no Lelouch",
            poster: "https://shikimori.io/system/animes/original/1575.jpg",
            score: "8.70",
            pts: 97,
            episodes: 25,
            description: "Опальный британский принц Лелуш получает мистическую силу абсолютного подчинения — Гиас — и возглавляет повстанческое движение против священной Британской империи.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Reanimedia",
        },
        PresetAnime {
            id: 16498,
            title: "Атака титанов",
            name: "Shingeki no Kyojin",
            poster: "https://shikimori.io/system/animes/original/16498.jpg",
            score: "8.55",
            pts: 97,
            episodes: 25,
            description: "Остатки человечества укрылись за тремя гигантскими концентрическими стенами от титанов-людоедов. Эрен Йегер клянется уничтожить каждого титана после падения родного города.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] Studio Band",
        },
        PresetAnime {
            id: 2001,
            title: "Гуррен-Лаганн",
            name: "Tengen Toppa Gurren Lagann",
            poster: "https://shikimori.io/system/animes/original/2001.jpg",
            score: "8.63",
            pts: 97,
            episodes: 27,
            description: "Симон и Камина вырываются из подземной деревни на поверхность Земли и ведут человечество сквозь звезды, пронзая небеса своим несломимым духом.",
            release: "[BDRip 1080p HEVC FLAC] Reanimedia",
        },
        PresetAnime {
            id: 30,
            title: "Евангелион",
            name: "Neon Genesis Evangelion",
            poster: "https://shikimori.io/system/animes/original/30.jpg",
            score: "8.36",
            pts: 96,
            episodes: 26,
            description: "14-летний Синдзи Икари вынужден пилотировать биомеханическую Еву-01 для защиты Токио-3 от нашествия загадочных Ангелов, исследуя глубины собственного экзистенциального кризиса.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] MC Entertainment",
        },
        PresetAnime {
            id: 1,
            title: "Ковбой Бибоп",
            name: "Cowboy Bebop",
            poster: "https://shikimori.io/system/animes/original/1.jpg",
            score: "8.75",
            pts: 98,
            episodes: 26,
            description: "Команда охотников за головами на звездолете 'Бибоп' бороздит Солнечную систему 2071 года под звуки джаза и блюза, не в силах сбежать от призраков своего прошлого.",
            release: "[BDRip 1080p HEVC 10-bit FLAC] AniLibria",
        },
        PresetAnime {
            id: 19,
            title: "Монстр",
            name: "Monster",
            poster: "https://shikimori.io/system/animes/original/19.jpg",
            score: "8.88",
            pts: 98,
            episodes: 74,
            description: "Талантливый нейрохирург Кэндзо Тэмма спасает жизнь тяжелораненому мальчику Йохану Либерту, не подозревая, что взрастил гениального психопата и монстра во плоти.",
            release: "[BDRip 1080p HEVC FLAC] AniDUB",
        },
    ]
}

pub fn all_preset_anime() -> Vec<PresetAnime> {
    let mut all = get_preset_ongoing();
    all.extend(get_preset_similar());
    all.extend(get_preset_ranked());
    all
}

pub fn find_preset_by_id(id: u64) -> Option<PresetAnime> {
    all_preset_anime().into_iter().find(|a| a.id == id)
}

pub fn search_presets(query: &str) -> Vec<PresetAnime> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    all_preset_anime()
        .into_iter()
        .filter(|a| a.title.to_lowercase().contains(&q) || a.name.to_lowercase().contains(&q))
        .collect()
}
