# kobo_zh_dict

Chinese-English dictionary builder for Kobo e-readers.


## Installation Instructions
*Tested on Kobo Libra 2*

1. Go to [Releases](https://github.com/kylefeng28/kobo_zh_dict/releases) and download the version that you want (simplified/traditional; pinyin/zhuyin)
2. Connect your Kobo and put the downloaded dictionary in `.kobo/custom-dicts`
3. **IMPORTANT**: You must rename the file to `dicthtml-ja-en.zip` to make it appear as a Japanese-English dictionary. Even though this is a Chinese-English dictionary, we have to fool the Kobo into thinking it is a Japanese-English dictionary. Otherwise, this will not work! (see explanation below)
  - If you already have a real Japanese-English dictionary, you can use another language pair, e.g. `dicthtml-ja-it.zip` for Japanese-Italian and `dicthtml-ja-es.zip` for Japanese-Spanish. However, the source language must be `ja` (Japanese).
4. Eject your Kobo and open a Chinese book and select a word. In the dictionary lookup, make sure to select `日本語-English (Custom)`. If this doesn't appear, something might have gone wrong.


### Why do weJapanese-English? Why can't we have it as Chinese-English dictionary?
**Short/simplified answer**: We can easily build a Chinese-English dictionary using a Stardict dictionary and [pyglossary](https://github.com/ilius/pyglossary), but it will be extremely slow.

Since Japanese/Chinese have thousands of characters whereas almost every other language has an alphabet of some sort, looking up words is slightly different compared to English/other languages. e.g. in English, to look up "disestablishmentarianism", we just have to look up the "d" section and proceed alphabetically until we find our word. This doesn't work for Japanese/Chinese, so the Kobo developers made a more efficient lookup method for Japanese.

However, the Kobo developers only enabled this for Japanese, but forgot to do it for Chinese. So to get the benefits of the efficient dictionary lookup for Chinese, we have fool Kobo into thinking that it is a Japanese dictionary. Otherwise, looking up a Chinese word will require looking through hundreds of thousands of words until it finds the right one, and is extremely slow on a small device like a Kobo.

**Longer/more technical answer**:

The explanation above is a bit oversimplified. Kobo actually tries to index words based on the first 2 letters/characters, like in a filing cabinet/encyclopedia volumes. These are called "prefixes". e.g. when you look up the word "disestablishmentarianism", Kobo will look in the "di" prefix, and find the word in there. The Kobo dictionary format is based on taking the entire dictionary and partitioning them into 2-character prefixes. These "prefixes" are stored as separate files in a zip file with the name format `dicthtml-*.zip`

For most languages (English, Spanish, Russian, etc) there are only a set number of letters, so this works pretty well. e.g. 26 letters for English => 676 possible prefixes.

However, for Japanese and Chinese, there are thousands of kanji/hanzi, so building a 2-character index will result in hundreds of thousands of prefixes (e.g. 136,376). This is extremely slow and inefficient for a small device like a Kobo, since each prefix will only have a small number of words, but there are tons of prefixes that individually are almost empty. Retrieving hundreds of thousands of tiny files in a zip file is extremely taxing on a small device like a Kobo.

Instead, Kobo uses a 1-character index for Japanese (among other things) which optimizes the lookup process and speeds up the process tremendously. However, as mentioned above, this is only for Japanese, and not for Chinese, and we must fool the device into thinking we are using a Japanese-English dictionary.

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
- [cessen/kobo_jp_dict](https://github.com/cessen/kobo_jp_dic) - Similar project for building Japanese-English dictionaries
- [dictutil](https://pgaskin.net/dictutil/) - Kobo dictionary tool
    - The [Dictionary format](https://pgaskin.net/dictutil/dicthtml/format.html) page here is very helpful in explaining the entry format
- tshering's Custom Chinese-English dictionary (`cedict4kobo.zip`) from this [MobileRead thread](https://www.mobileread.com/forums/showthread.php?t=202182)

