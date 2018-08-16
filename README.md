## Safepdf


This is a Rust implementation of Micah Lee's famous [pdf-redact-tools](https://github.com/firstlookmedia/pdf-redact-tools).

We are here using Docker containers to isolate the environment where we
actually read/open up the PDF file.

*Warning:* A malicious PDF could exploit a bug in GraphicsMagick to take over
your computer. If you're working with potentially malicious PDFs, it's safest
to run them through PDF Redact Tools in an isolated environment, such as a
virtual machine, or by using a tool such as the [Qubes PDF
Converter](https://github.com/QubesOS/qubes-app-linux-pdf-converter) instead.


## Dependencies

- GraphicsMagick
- ghostscript-core (On Debian: ghostscript)
- perl-Image-ExifTool (On Debian: libimage-exiftool-perl)
- docker


### How to use?

We have a Fedora 28 bsaed Dockerfile in the repo, just to be safe we should always build
the container image in our system. Remember that here my normal user can use docker commands.

```
docker build -t donotupload/safepdf .
```

Then, you can use the `run.sh` provided in this repository.

```
$ ./run.sh why_johny_cant_encrypt.pdf
Creating images from the PDF.
Verifing the images and then joining back into the final PDF.
```


#### License: GPLv3+
