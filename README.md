# mod_vig

---
## About

This is a simple cli-application using crates
- modernized_vigenere
- clap

Please check https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher for a brief definition of the Vigenere cipher.\
As you can see, this encryption/decryption is based on shifting the letters, based on a en/de-cryption phrase.\
But time has changed since the 16th century:\
  The dominent way to pass information, is not via paper (using the alphabetic letters) any more.\
\
Today, we use files ( and not only ASCII-files ).\
So this modernized lib does not shift alphabetic letters but bytes !

Therefore this lib can be used to en/de-crypt not only ASCII-files, but any kind of files (yes, even binaries),\
and is able to use not only Strings but also (binary) files as a phrase for en/de-cryption.

---
## Usage

```
$ mod_vig --help
mod_vig 0.1.0
Marty3000
cli for a modernized Vigenere algorithm

USAGE:
    mod_vig [OPTIONS] --ifile <IFILE> --phrase <PHRASE> --ofile <OFILE>

OPTIONS:
    -d, --decrypt            encrypt or decrypt
    -h, --help               Print help information
    -i, --ifile <IFILE>      Name of the input file
    -o, --ofile <OFILE>      Name of the output file
    -p, --phrase <PHRASE>    cipher phrase-file or cipher phrase
    -v, --verbose            print verbose messages
    -V, --version            Print version information

$ mod_vig -i /tmp/CatContent.pdf -p "This is the cipher-phrase" -o /tmp/CatContent.encrypted -v
  encrypting /tmp/CatContent.pdf into /tmp/CatContent.encrypted ...
  Time: 0.014832271 seconds

$ mod_vig -d -i /tmp/CatContent.encrypted -p "This is the cipher-phrase" -o /tmp/CatContent.decrypted -v
  decrypting /tmp/CatContent.encrypted into /tmpCatContent.decrypted ...
  Time: 0.014272052 seconds

  $ cksum  /tmp/CatCont*
  1821036750 1194280 /tmp/CatContent.decrypted
  1466043786 1194280 /tmp/CatContent.encrypted
  1821036750 1194280 /tmp/CatContent.pdf

  $ file /tmp/CatCont*
  /tmp/CatContent.decrypted: PDF document, version 1.6
  /tmp/CatContent.encrypted: data
  /tmp/CatContent.pdf:       PDF document, version 1.6
```
---
## Greetings

Thank you too everyone who participated in the development of rust, cargo, atom or any crate.
