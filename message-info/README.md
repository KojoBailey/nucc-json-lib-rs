# NUCC MessageInfo JSON
## Spec
All fields use [`snake_case`](https://en.wikipedia.org/wiki/Snake_case).

The `filetype` field is required to be `"MessageInfo"`.

Currently, there is only one version of this format, so you can use your current date (`YYMMDD`) for the `version` field.

The `language` field should be one of the following as a string:
- `English`
- `Spanish`
- `German`
- `Italian`
- `French`
- `Japanese`
- `Korean`
- `ChineseSimplified`
- `ChineseTraditional`

The `colors` field takes a map of symbols to colours of the format `#rrggbbaa`. As of now, there are no built-in presets, but it may be considered in future.

The key for each entry can either be the string's ID, like `1jnt01_btlst_00_1dio01`, or the CRC-32 hash of that ID, like `47f11ca9`.

The contents for each entry are as follows:

| Field | Type | Required? | Description | Example |
|-------|------|-----------|-------------|---------|
| `message` | String | No | Corresponding string. | `I'll kill your mum, too!` |
| `reference` | String | No | The ID of another string to copy its message instead. | `2lsa01_btlcmd_sp_01` |
| `adx2_file` | String | No | The [ADX2](https://jojomodding.miraheze.org/wiki/ADX2) file in which the audio is stored. | `v_btl_8wou01` |
| `adx2_cue_index` | Int | No | If corresponding to a voiceline, the index of that voiceline in its ADX2 file. | `102` |

## Example
> [!warning]
> The comments below are invalid and unsupported by JSON officially-speaking.

```jsonc
{
    "version": 260223,
    "filetype": "MessageInfo",
    "language": "English",
    "colors": {
        "r": "#f73939ff"
    },

    "1jnt01_btlst_00_1dio01": {
        "message": "I will put an end to this here and now, <r>Dio</r>!",
        "adx2_file": "v_btl_1jnt01",
        "adx2_cue_index": 23
    },
    "10038683": { // 5grn01_btlwin_00_6pci01
        "message": "You knew my father? He must've gotten around...",
        "adx2_file": "v_btl_5grn01",
        "adx2_cue_index": 117
    },
    "6wet01_btlcmd_name_gha": {
        "reference": "campaign_support_t011"
    }
}
```
