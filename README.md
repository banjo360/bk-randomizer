# Banjo-Kazooie Randomizer for the Xbox 360

This is still an early work-in-progress (i.e. "unusable as-is"). "main.rs" changes based on my tests and progress.

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

`worlds` (bool): Shuffle the world order or not.

`moves` (bool): Unlock ALL moves from the start.

`notedoors` (array of int): List of note doors that will be opened when starting a new game.

`pipes` (bool): Raise the pipes in Clanker's Cavern's lobby.

`cauldrons` (bool): All cauldrons start already active.

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

You can shuffle [any actor](src/enums/actors.rs), but some like `Conga` might *break* the game or make some things impossible to get.

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
