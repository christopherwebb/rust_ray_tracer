from noise import pnoise2


def terrain(gen_height, gen_width):
    result = []
    for y in gen_height:
        row = []
        for x in gen_width:
            height = pnoise2(
                x / 16.0, y / 16.0, 16.0)
            row.append((height, height, height))

        result.append(row)

    return row


def save_ppm6(data):
    with open("result.ppm", "wb+") as f:
        f.write('p6\n')
        f.write('Generated heightfield\n')

        f.write(f"{width} {height}\n")

        for row in data:
            for height in row:
                red = height[0] * 255
                green = height[1] * 255
                blue = height[2] * 255
                f.write(f"{red} {green} {blue}\n")

if __name__ == "__main__":
    result = terrain(1024, 1024)
    save_ppm(result)
