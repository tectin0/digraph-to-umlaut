# digraph-to-umlaut

This repository contains an executable written in Rust that allows you to convert words in a single file or multiple files within a folder. It replaces all instances of digraph equivalents of umlauts (ae, ue, oe) with the corresponding umlaut character. It also tries to replace "ss" with "ß" if the word with "ss" does not exist.

## Usage

### DISCLAIMER: This is a work in progress, not all words are converted correctly, not all edge cases are handled. Use at your own risk. It is intended to be used for converting (most) German words to the correct spelling when using a non-German keyboard layout. Shortcuts were taken to not having to deal with the dictionary affix file.

### Option 1

1. Download the latest zip file from the [releases page](https://github.com/tectin0/digraph-to-umlaut/releases) and extract it.
   - This should contain the `config.toml` file, `data` folder and the `digraph-to-umlaut.exe` executable
   - The `data` folder contains the dictionaries (list of words) used by the program
     - There is one special dictionary the `exclude.dic` which is a list of words that are ignored by the converter
     - Additional dictionaries can be added: They must end in `.dic` and `#` are treated as comments
2. The program can be started in a terminal with: `digraph-to-umlaut.exe -i <path-to-a-folder-or-file>`
3. This creates an `output` folder in the same directory as `digraph-to-umlaut.exe`. I advise to check the output files before replacing the original files.

### Option 2

1. Clone the repository and run `cargo run --release -- -i <path-to-a-folder-or-file>`

### Additional Information

If some weird characters appear in the output files, make sure that the input files are encoded in UTF-8. If you are using VS Code, you can change the encoding by clicking on the encoding in the bottom right corner of the editor.

I used the python file `scrape_wiktionary.py` to extract a list of chemistry words for the `chem_terms.dic`

An example file can be found in the `samples` folder and the corresponding output can be found in the `output` folder of the repository (they are not included in the release)

Includes a VS Code tasks.json with a default build task that runs the program on the `samples` folder.

## Credits

- [LibreOffice](https://github.com/LibreOffice/dictionaries) for the de_DE dictionary
- [Wiktionary](https://de.wiktionary.org/wiki/Verzeichnis%253ADeutsch/Chemie/Fachwortliste) for the list of chemistry words
