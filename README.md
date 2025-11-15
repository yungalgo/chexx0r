# chexx0r

```
┎┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈୨♡୧┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┒
┃                                                    ┃
┃           ✧･ﾟ: *✧･ﾟ:* chexx0r *:･ﾟ✧*:･ﾟ✧             ┃
┃                                                    ┃
┃          simple cli utility for checking           ┃
┃          domain and username availability          ┃
┃                                                    ┃
┖┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈୨♡୧┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┈┚
```

---

## why i built this

```
┌─────────────────────────── •✧✧• ───────────────────────────┐
┃  whenever i start a new webapp, game, or project,          ┃
┃  i need to check domain and social media availability      ┃
┃  because it affects what i name it.                        ┃
┃                                                            ┃
┃  before chexx0r, i had to manually check:                  ┃
┃    • instagram (username)                                  ┃
┃    • tiktok (@username)                                    ┃
┃    • youtube (@username)                                   ┃
┃    • godaddy for domains                                   ┃
┃                                                            ┃
┃  this took forever and was super tedious.                  ┃
└─────────────────────────── •✧✧• ───────────────────────────┘

```

## features

```
┏◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚◚┓
┃  ✦ domain availability checking                              ┃
┃    • multiple TLD presets (startup, enterprise, country)     ┃
┃    • custom TLD support                                      ┃
┃    • RDAP + WHOIS integration                                ┃
┃                                                              ┃
┃  ✦ social media platform checking                            ┃
┃    • YouTube (@username)                                     ┃
┃    • Instagram (username)                                    ┃
┃    • TikTok (@username)                                      ┃
┃                                                              ┃
┃  ✦ beautiful terminal ui                                     ┃
┃    • decorative kaomojis and ascii art boxes                 ┃
┃    • color-coded status indicators                           ┃
┃    • animated progress spinners                              ┃
┃    • sparkle-filled decorative patterns                      ┃
┗◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛◛┛

```

---

## installation & build

```
╔══════════════════════════════░⋆ ✪ ⋆░══════════════════════════════╗
║ ✧･ﾟ: *✧･ﾟ:*                  prereqs                   ✧･ﾟ: *✧･ﾟ:* ║
╚══════════════════════════════░⋆ ✪ ⋆░══════════════════════════════╝

```

**install rust** (if not already installed):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

**build & install:**
```bash
# clone repo
git clone https://github.com/yourusername/chexx0r.git
cd chexx0r

# build release version (recommended)
cargo build --release

# install globally
cargo install --path .
```

**verify installation:**
```bash
chexx0r --help
```

---

## quickstart

```
╭═───────────────────────────────═⌘═─────────────────────────────═╮
│ ⋆ ˚｡⋆୨୧˚                   basic usage                  ⋆ ˚｡⋆୨୧˚ │
╰═───────────────────────────────═⌘═─────────────────────────────═╯

```

```bash
# check username availability (default: startup preset)
chexx0r jeffreyepstein

# with custom TLDs
chexx0r cameronwinter --tlds com,io,dev,ai

# enterprise tlds preset
chexx0r fortnite4ever --preset enterprise

# country-specific tlds
chexx0r xXJeremyClementinesXx --preset country

# only check domains (skip social media)
chexx0r sweetychat4lyfe --skip-social

# only check social media (skip domains)
chexx0r mamdaniluvr69 --skip-domains
```

---

## full capabilities

### domain checking

```
┏━━━━━━━━━━━━━━━━━━━━━━━━┅┅┄┄⟞⟦✮⟧⟝┄┄┉┉━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ ⭒❃.✮:▹ ◃:✮.❃⭒            tld presets          ⭒❃.✮:▹ ◃:✮.❃⭒ ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━┅┅┄┄⟞⟦✮⟧⟝┄┄┉┉━━━━━━━━━━━━━━━━━━━━━━━━┛
```

- **`startup`** (default): `com`, `org`, `io`, `ai`, `tech`, `app`, `dev`, `xyz`
- **`enterprise`**: `com`, `org`, `net`, `info`, `biz`, `us`
- **`country`**: `us`, `uk`, `de`, `fr`, `ca`, `au`, `jp`, `br`, `in`
- **custom**: specify any TLDs with `--tlds com,net,org,io`

**status indicators:**
- **AVAILABLE** - domain is available for registration
- **TAKEN** - domain is already registered
- **UNKNOWN** - unable to determine availability

### social media platform checking

```
╔══════════════════════════════✦❖✦═════════════════════════════╗
║ ‧͙⁺˚･༓☾ ☽༓･˚⁺‧͙             platforms             ‧͙⁺˚･༓☾ ☽༓･˚⁺‧͙ ║
╚══════════════════════════════✦❖✦═════════════════════════════╝

```

**1. YouTube** (`@username`)
   - validates username format (3-20 chars, alphanumeric, hyphens, underscores)
   - checks channel availability via HTTP status codes

**2. Instagram** (`username`)
   - validates username format (1-30 chars, letters, numbers, periods, underscores)
   - detects profile existence via HTML title tag analysis

**3. TikTok** (`@username`)
   - validates username format (1-24 chars, must start with letter)
   - checks embedded JSON data for profile existence

**status indicators:**
- **AVAILABLE** - username is available
- **TAKEN** - username is already in use
- **UNKNOWN** - unable to determine status
- **INVALID** - username format is invalid for the platform

---

## command line options

```
╭━━━━━━━━━━━━━━━━━━━━━━━━━───── • ◈ • ─────━━━━━━━━━━━━━━━━━━━━━━━━╮
│                  chexx0r [OPTIONS] <USERNAME>                    │
│                                                                  │
│                           arguments:                             │
│                 <USERNAME>    username to check                  │
│                                                                  │
│                            options:                              │
│                      -p, --preset <PRESET>                       │
│             TLD preset: startup, enterprise, country             │
│                       [default: startup]                         │
│                                                                  │
│                        -t, --tlds <TLDS>                         │
│                   custom tlds (comma-separated)                  │
│                  example: --tlds com,org,io,dev                  │
│                                                                  │
│                          --skip-domains                          │
│                  skip domain availability checks                 │
│                                                                  │
│                          --skip-social                           │
│               skip social media availability checks              │
│                                                                  │
│                             --debug                              │
│             show debug output for social media checks            │
│                                                                  │
│                           -h, --help                             │
│                     print help information                       │
╰━━━━━━━━━━━━━━━━━━━━━━━━━───── • ◈ • ─────━━━━━━━━━━━━━━━━━━━━━━━━╯

```

---

## usage example

```bash
$ chexx0r yungalgo
```

**output:**
```
┎╌╌╌╌╌╌╌╌╌╯╰╌╌╌╌╌╌╌╌╌┒
┃ checking: yungalgo ┃
┖╌╌╌╌╌╌╌╌╌╮╭╌╌╌╌╌╌╌╌╌┚

╔════════━━━─── • ───━━━════════╗
║            domains            ║
╠════════━━━─── • ───━━━════════╣
║ yungalgo.com       AVAILABLE  ║
║ yungalgo.org       AVAILABLE  ║
║ yungalgo.io        AVAILABLE  ║
║ yungalgo.ai        AVAILABLE  ║
╚════════━━━─── • ───━━━════════╝

╔════════━━━─── • ───━━━════════╗
║            social             ║
╠════════━━━─── • ───━━━════════╣
║ youtube            TAKEN      ║
║ instagram          AVAILABLE  ║
║ tiktok             AVAILABLE  ║
╚════════━━━─── • ───━━━════════╝
╔════════━━━─── • ───━━━════════╗
║           complete            ║
╚════════━━━─── • ───━━━════════╝

```

---

## architecture

```
┯━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ●●● ━━━━━━━━━━━━━━━━━━━━━━━━━━━┯
┃                clean separation of concerns                ┃
┃                                                            ┃
┃  main.rs      → cli orchestration & progress               ┃
┃  domain.rs    → domain availability logic                  ┃
┃  social.rs    → social media checking logic                ┃
┃  ui.rs        → beautiful terminal rendering               ┃
┃  config.rs    → platform & tld configurations              ┃
┃  utils.rs     → username validation functions              ┃
┃  lib.rs       → public api for testing                     ┃
┷━━━━━━━━━━━━━━━━━━━━━━━━━━━━ ●●● ━━━━━━━━━━━━━━━━━━━━━━━━━━━┷
```

---

## built with

```
╔═════════════════════════════¤◎¤═══════════════════════════════╗
║ ☆.｡.: .｡.:☆              dependencies              ☆.｡.: .｡.:☆ ║
╚═════════════════════════════¤◎¤═══════════════════════════════╝
```

- **[domain-check-lib](https://github.com/saidutt46/domain-check)** - domain availability checking via rdap/whois
- **[reqwest](https://github.com/seanmonstar/reqwest)** - async http client for social media checks
- **[clap](https://github.com/clap-rs/clap)** - command-line argument parsing
- **[tokio](https://github.com/tokio-rs/tokio)** - async runtime
- **[comfy-table](https://github.com/Nukesor/comfy-table)** - beautiful terminal tables
- **[colored](https://github.com/mackwic/colored)** - terminal colors
- **[indicatif](https://github.com/console-rs/indicatif)** - progress bars and spinners
- **[kaomojis, ascii art and other text dividers](https://gist.github.com/jamiew/40c66061b666272462c17f65addb14d5)** - source for decorative kaomojis and ascii art patterns

---

## development

```
╭━─━─━─━─━─━─━─━─━─━─━─━─━─━─━≪✠≫━─━─━─━─━─━─━─━─━─━─━─━─━─━╮
│ .｡❅⋆⍋∞｡∞⍋⋆❅｡.      development commands       .｡❅⋆⍋∞｡∞⍋⋆❅｡. │
╰━─━─━─━─━─━─━─━─━─━─━─━─━─━─━≪✠≫━─━─━─━─━─━─━─━─━─━─━─━─━─━╯
```

```bash
# run tests
cargo test

# format code
cargo fmt

# lint code
cargo clippy

# clean build artifacts
cargo clean

# update dependencies
cargo update
```

---

## license

```
╒═════════════════╕
║ ♥ mit license♥  ║
╘═════════════════╛
```

mit license - feel free to use this project for your own purposes!

---

## contributing

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃                    contributors welcomed                   ┃
┃                                                            ┃
┃                       1. fork the repo                     ┃
┃                  2. create a feature branch                ┃
┃                     3. make your changes                   ┃
┃                 4. run tests and formatting                ┃
┃                5. submit pr to merge into main             ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

---
