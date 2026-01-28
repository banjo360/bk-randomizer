# Banjo-Kazooie Randomizer for the Xbox 360

This is still an early work-in-progress (i.e. "unusable as-is"). "main.rs" changes based on my tests and progress.

## Preparation

- Extract the files from Banjo-Kazooie with [wxPirs](https://digiex.net/threads/wxpirs-extract-content-from-xbox-360-demos-video-dlc-and-arcade-game-containers.9464/).
- Decrypt `default.xex` with [xextool](https://digiex.net/threads/xextool-6-3-download.9523/): `xextool.exe -e u default.xex`.
- Decrypt `RAWFiles/db360.cmp` with `xbdecompress.exe` from the Xbox 360 SDK: `xbdecompress.exe db360.cmp db360.cmp` (say "Yes" when asked if you want to overwrite)
