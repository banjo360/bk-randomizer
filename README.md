# Banjo-Kazooie Randomizer for the Xbox 360

This is still an work-in-progress. There is a possibility of a game crash or a softlock. Don't hesitate to report them.

## Preparation

- Extract the files from Banjo-Kazooie with [wxPirs](https://digiex.net/threads/wxpirs-extract-content-from-xbox-360-demos-video-dlc-and-arcade-game-containers.9464/).
- Decrypt `default.xex` with [xextool](https://digiex.net/threads/xextool-6-3-download.9523/): `xextool.exe -e u default.xex`.
- Decrypt `RAWFiles/db360.cmp` with `xbdecompress.exe` from the Xbox 360 SDK: `xbdecompress.exe db360.cmp db360.cmp` (say "Yes" when asked if you want to overwrite)
- Decrypt `RAWFiles/db360.textures.cmp` the same way
- All 3 files should be in the same directory
- Need a `config.json` file containing the configuration you want.

## config.json

`actors`: a list of the actors to shuffle.

`sprites`: a list of 2D objects to shuffle.

`mix` (bool): if `true`, `actors` and `sprites` are shuffled together (i.e. an egg can be swapped with a jiggy), otherwise, `actors` are only shuffled between themselves, and same for `sprites`.

`worlds` (bool): Shuffle the world order if `true`.

`moves` (bool): Unlock ALL moves from the start if `true`.

`notedoors` (array of int): List of note doors that will be already opened when starting a new game. (possible values: 50, 180, 260, 350, 450, 640, 765, 810, 828, 846, 864, 882)

`pipes` (bool): Raise the pipes in Clanker's Cavern's lobby, and open the grate to BGS' puzzle.

`cauldrons` (bool): All cauldrons start already active.

`enemies` (bool): Randomize enemies in the whole game. (they are not shuffled, each enemy is replaced by another one at random)

`skip_furnace_fun` (bool): Skips Furnace Fun. The board is still there but its state is set to "beaten".

`easy_talon_trot` (bool): You stay in Talon Trot without having to keep a trigger pressed. Press a trigger again to exit Talon Trot. \
⚠️ currently not working

### Available actors

This is a list of *safe* actors to shuffle:

- `Beehive`
- `BlubbersGold`
- `BlueJinjo`
- `ChimpysOrange`
- `CollectableBluePresent`
- `CollectableGreenPresent`
- `CollectableRedPresent`
- `EmptyHoneycomb`
- `ExtraLife`
- `GreenJinjo`
- `Jiggy`
- `MmmFlowerPot`
- `MumboToken`
- `NabnutsAcorn`
- `OrangeJinjo`
- `PinkJinjo`
- `YellowJinjo`

You can shuffle [any actor](src/enums/actors.rs) that are not enemies (e.g. Teehee, Grublin, etc), but some like `Conga` might *break* the game or make some things impossible to get.

### Available sprites

- `BlueEgg`
- `BlueTulip`
- `ConchShell`
- `GoldFeather`
- `MusicalNote`
- `OrangeDaisy`
- `RedDaisy`
- `RedFeather`
- `ThickSeaweed`
- `ThinSeaweed`

## Build

You need the [ppc](https://github.com/minirop/ppc) crate and, maybe, update its path in `Cargo.toml`.
