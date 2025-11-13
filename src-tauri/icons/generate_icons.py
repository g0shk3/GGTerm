#!/usr/bin/env python3
from PIL import Image, ImageDraw, ImageFont

# Създаваме иконка с тъмно синьо и бял текст "GG"
sizes = [32, 128, 256, 512, 1024]

for size in sizes:
    img = Image.new('RGBA', (size, size), (139, 233, 253, 255))  # Cyan от темата
    d = ImageDraw.Draw(img)

    # Добавяме текст "GG" в центъра
    font_size = size // 3
    try:
        # Опитваме се да намерим system font
        font = ImageFont.truetype("/System/Library/Fonts/Helvetica.ttc", font_size)
    except:
        try:
            font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", font_size)
        except:
            font = ImageFont.load_default()

    # Центрираме текста
    text = "GG"
    bbox = d.textbbox((0, 0), text, font=font)
    text_width = bbox[2] - bbox[0]
    text_height = bbox[3] - bbox[1]
    x = (size - text_width) // 2
    y = (size - text_height) // 2 - font_size // 10

    d.text((x, y), text, fill=(30, 30, 46, 255), font=font)

    # Запазваме
    if size == 1024:
        img.save(f'icon.png')
    else:
        img.save(f'{size}x{size}.png')

    print(f"Created {size}x{size}.png")

# Създаваме и @2x версия
img_256 = Image.open('256x256.png')
img_256.save('128x128@2x.png')
print("Created 128x128@2x.png")

print("\nAll icons created successfully!")
