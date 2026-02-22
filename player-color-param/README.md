# NUCC PlayerColorParam JSON
## Spec
All fields use [`snake_case`](https://en.wikipedia.org/wiki/Snake_case).

The `filetype` field is required to be `"PlayerColorParam"`.

Currently, there is only one version of this format, so you can use your current date (`YYMMDD`) for the `version` field.

Each entry key is a unique code specifying the character's ID, costume index, and alt index - as seen in the game's `data/spc` directory. For example, Jotaro's Special Costume C Tint Colour 2 would be `3jtr31col1`.

All they store is a hex colour code of the format `#RRGGBB`.

## Example
> [!warning]
> The comments below are invalid and unsupported by JSON officially-speaking.

```jsonc
{
    "filetype": "PlayerColorParam",
    "version": 240930,

    "1jnt31col1": "#FF0000", // Jonathan Joestar, Special Costume C, Tint Colour 2
    "5grn01col0": "#FF0000", // Giorno Giovanna, Default Costume, Tint Colour 1
    "5grn01col1": "#DA8A00", // Giorno Giovanna, Default Costume, Tint Colour 2
    "5grn01col2": "#D6DA00", // Giorno Giovanna, Default Costume, Tint Colour 3
    "5grn01col3": "#1DDA00", // Giorno Giovanna, Default Costume, Tint Colour 4
    "5grn11col0": "#00DADA", // Giorno Giovanna, Special Costume A, Tint Colour 1
    "5grn11col1": "#0700DA", // Giorno Giovanna, Special Costume A, Tint Colour 2
    "5grn21col0": "#9900DA", // Giorno Giovanna, Special Costume B, Tint Colour 1
    "5grn21col1": "#DA0091"  // Giorno Giovanna, Special Costume B, Tint Colour 2
}
```
