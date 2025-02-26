# Bible API

A single-dependency and easy-to-use API to interact with the Bible.

I am creating this to I use in various projects of my own, but am publishing it here for the benefit of anyone who would use it.

I have dreams about supporting [PyO3](https://github.com/PyO3/pyo3) for Python usage, [Web Assembly](https://rustwasm.github.io/wasm-bindgen/introduction.html) for TypeScript/JavaScript usage, and maybe even a REST API for language agnostic interaction, but I will see what the Lord wills.

## Features

> Some of these are work in progress

- Extremely fast and memory efficient 🦀
- Parse complex Bible references (such as `Ephesians 1:1-2,4-6; 22-2:2,5; 3:21-4:2`, even with book abbreviations and inconsistent spacing and comma/semi-colon usage)
- Ergonomic interfaces for interacting with passage ranges of all kinds
- Specify output format for parsed Bible passages with templates
- Find all Bible references and their locations (for, but not limited to, usage by an LSP)
- Suggest/autocomplete verses from a reference

## Source Data

The input JSON file (currently) should look something like this.

This is likely subject to change as I desire to add cross-references, an inter-linear, headings, paragraphs, indentation, and so on.
I am thinking potentially of the `content` property, though I may use another property for this data.

```jsonc
{
  "translation": {
    "name": "English Standard Version",
    "language": "English",
    "abbreviation": "ESV"
  },
  "bible": [
    {
      "id": 1,
      "book": "Genesis",
      "abbreviations": [
        "gen",
        "ge",
        "gn"
      ],
      "content": [
        [
          "In the beginning, God created the heavens and the earth.",
          "The earth was without form and void, and darkness was over the face of the deep. And the Spirit of God was hovering over the face of the waters.",
          // remaining verses in Genesis 1 ...
        ],
        [
          "Thus the heavens and the earth were finished, and all the host of them.",
          "And on the seventh day God finished his work that he had done, and he rested on the seventh day from all his work that he had done.",
          // remaining verses in Genesis 2 ...
        ]
      ]
    },
    // remaining books of the Bible
  ]
}
```
