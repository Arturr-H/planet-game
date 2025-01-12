from PIL import Image
import os
import sys

def extract_textures(image_path, texture_width, texture_height, start_index, output_dir):
    try:
        # Open the image
        atlas = Image.open(image_path)
        
        # Get the dimensions of the atlas
        atlas_width, atlas_height = atlas.size

        # Calculate the number of textures
        cols = atlas_width // texture_width
        rows = atlas_height // texture_height

        if not os.path.exists(output_dir):
            os.makedirs(output_dir)

        current_index = start_index

        for row in range(rows):
            for col in range(cols):
                # Define the box to crop
                left = col * texture_width
                upper = row * texture_height
                right = left + texture_width
                lower = upper + texture_height

                # Crop the region and save it
                cropped = atlas.crop((left, upper, right, lower))
                output_path = os.path.join(output_dir, f"{current_index}.png")
                cropped.save(output_path)
                print(f"Saved: {output_path}")

                current_index += 1

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 6:
        print("Usage: python script.py <image_path> <texture_width> <texture_height> <start_index> <output_dir>")
        sys.exit(1)

    image_path = sys.argv[1]
    texture_width = int(sys.argv[2])
    texture_height = int(sys.argv[3])
    start_index = int(sys.argv[4])
    output_dir = sys.argv[5]

    extract_textures(image_path, texture_width, texture_height, start_index, output_dir)
