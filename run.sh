#!/bin/bash

mkdir -p pdfworkplace
cp $1 pdfworkplace/
filename=`basename $1`
dir_name=`dirname $1`
echo "Creating images from the PDF."
docker  run --rm --network=none -v $PWD/pdfworkplace:/home/pdfuser/workplace dontupload/safepdf safepdf -e -c $filename
echo "Verifing the images and then joining back into the final PDF."
docker  run --rm --network=none -v $PWD/pdfworkplace:/home/pdfuser/workplace dontupload/safepdf safepdf -j
final="${filename%.*}"
cp pdfworkplace/$final-final.pdf $dir_name/

