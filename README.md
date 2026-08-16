# kobo_zh_dict

Chinese-English dictionary builder for Kobo e-readers.


## Installation Instructions
*Tested on Kobo Libra 2*

1. Go to [Releases](https://github.com/kylefeng28/kobo_zh_dict/releases) and download the version that you want (simplified/traditional; pinyin/zhuyin)
2. Connect your Kobo and put the downloaded dictionary in `.kobo/custom-dicts` as `dicthtml-zh-en.zip`
3. Eject your Kobo and open a Chinese book and select a word. In the dictionary lookup, make sure to select `简体中文-English (Custom)`. If this doesn't appear or the lookups are extremely slow, try the workaround below.

### Workaround: Pretend to be a Japanese dictionary
If the lookups are extremely slow, try renaming `dicthtml-zh-en.html` to `dicthtml-ja-en.html` to masquerade as a Japanese-English dictionary instead of a Chinese-English dictionary.

This works because Kobo optimizes dictionary lookups for Japanese, but not for Chinese. Newer versions of Kobo include support for a `prefix_exceptions` file which allows us to simulate this, but for older devices, we must fool the device into thinking we are using a Japanese-English dictionary.

For a more technical explanation of how these prefixes work, see these pages:
- https://pgaskin.net/dictutil/dicthtml/format.html
- https://pgaskin.net/dictutil/dicthtml/prefixes.html

## Developer Information
**Requirements**:
- A recent version of Rust/`cargo`
- [marisa-trie](https://github.com/s-yata/marisa-trie)

Automatic:
```
$ generate_dicts.sh
```

Manual:
```
# Download CC-CEDICT
$ curl -LO https://www.mdbg.net/chinese/export/cedict/cedict_1_0_ts_utf-8_mdbg.txt.gz
$ gunzip cedict_1_0_ts_utf-8_mdbg.txt.gz
$ cargo run -c simplified -p pinyin    # simplified with pinyin
$ cargo run -c traditional -p pinyin   # traditional with pinyin
$ cargo run -c traditional -p zhuyin   # traditional with zhuyin
```

Run `cargo run -- -h` to see all available CLI arguments.

## Acknowledgements
- [cessen/kobo_jp_dict](https://github.com/cessen/kobo_jp_dict) - Similar project for building Japanese-English dictionaries
- [dictutil](https://pgaskin.net/dictutil/) - Kobo dictionary tool
    - The [Dictionary format](https://pgaskin.net/dictutil/dicthtml/format.html) page here is very helpful in explaining the entry format
- tshering's Custom Chinese-English dictionary (`cedict4kobo.zip`) from this [MobileRead thread](https://www.mobileread.com/forums/showthread.php?t=202182)

