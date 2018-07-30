## Safepdf


This is a Rust implementation of Micah Lee's famous [pdf-redact-tools](https://github.com/firstlookmedia/pdf-redact-tools). This is still not
feature complete.

I am using this tool as a chance to learn Rust, so the code may be really
buggy. I am writing it for learning purpose.

*Warning:* PDF Redact Tools uses ImageMagick to parse PDFs. While ImageMagick
is a versatile tool, it has a history of some
[terrible](https://imagetragick.com/) security bugs. A malicious PDF could
exploit a bug in ImageMagick to take over your computer. If you're working
with potentially malicious PDFs, it's safest to run them through PDF Redact
Tools in an isolated environment, such as a virtual machine, or by using a
tool such as the [Qubes PDF
Converter](https://github.com/QubesOS/qubes-app-linux-pdf-converter) instead.


### How to use?

```
$ safepdf -c file.pdf
```

The above command will generate a new PDF called `file-safe.pdf`. The tool
requires ImageMagick to be installed on the system.

#### License: GPLv3+
