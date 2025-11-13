#!/bin/bash

# Минимален 1x1 transparent PNG (89 байта) - ще го scale-нем
echo "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==" | base64 -d > temp.png

# Използваме ImageMagick или sips (на macOS) за scale
if command -v sips &> /dev/null; then
    # macOS sips tool
    sips -z 32 32 temp.png --out 32x32.png 2>/dev/null
    sips -z 128 128 temp.png --out 128x128.png 2>/dev/null
    sips -z 256 256 temp.png --out 128x128@2x.png 2>/dev/null
    sips -z 512 512 temp.png --out icon.png 2>/dev/null
    echo "Icons created with sips"
elif command -v convert &> /dev/null; then
    # ImageMagick
    convert temp.png -resize 32x32 32x32.png
    convert temp.png -resize 128x128 128x128.png
    convert temp.png -resize 256x256 128x128@2x.png
    convert temp.png -resize 512x512 icon.png
    echo "Icons created with ImageMagick"
else
    # Fallback - просто копираме малкия файл
    cp temp.png 32x32.png
    cp temp.png 128x128.png
    cp temp.png 128x128@2x.png
    cp temp.png icon.png
    echo "No image tool found, using minimal placeholder"
fi

rm temp.png
ls -lh *.png
