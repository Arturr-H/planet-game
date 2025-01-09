import sys
import re
import numpy as np

def hex_to_rgba(hex_color):
    """Convert a hex color code to RGBA with values in [0.0, 1.0]."""
    hex_color = hex_color.lstrip('#')
    if len(hex_color) == 6:  # RGB
        r, g, b = tuple(int(hex_color[i:i+2], 16) / 255.0 for i in (0, 2, 4))
        a = 1.0
    elif len(hex_color) == 8:  # RGBA
        r, g, b, a = tuple(int(hex_color[i:i+2], 16) / 255.0 for i in (0, 2, 4, 6))
    else:
        raise ValueError("Invalid hex color format.")
    return np.array([r, g, b, a], dtype=np.float32)

def to_linear(nonlinear):
    """Convert an RGBA vector to linear space."""
    cutoff = nonlinear <= 0.04045
    higher = ((nonlinear + 0.055) / 1.055) ** 2.4
    lower = nonlinear / 12.92
    return np.where(cutoff, lower, higher)

def parse_color_input(color_input):
    # Check if the color is in the rgb or rgba format
    rgb_pattern = r'rgba?\(\s*(\d+\.?\d*),\s*(\d+\.?\d*),\s*(\d+\.?\d*)(?:,\s*(\d+\.?\d*))?\s*\)'
    hex_pattern = r'#([0-9a-fA-F]{6}|[0-9a-fA-F]{8})'

    # Match RGB or RGBA format
    match_rgb = re.match(rgb_pattern, color_input)
    if match_rgb:
        r, g, b = match_rgb.groups()[:3]
        a = match_rgb.group(4) if match_rgb.group(4) else 1.0
        
        # Check if any of the components are floats (range 0.0 to 1.0) or integers (0 to 255)
        if '.' in r or '.' in g or '.' in b or '.' in a:
            # Float range [0.0, 1.0]
            return np.array([float(r), float(g), float(b), float(a)], dtype=np.float32)
        else:
            # Integer range [0, 255]
            return np.array([int(r) / 255.0, int(g) / 255.0, int(b) / 255.0, float(a)], dtype=np.float32)

    # Match HEX format
    match_hex = re.match(hex_pattern, color_input)
    if match_hex:
        hex_value = match_hex.group(1)
        if len(hex_value) == 6:  # hex without alpha
            r, g, b = [int(hex_value[i:i+2], 16) for i in range(0, 6, 2)]
            return np.array([r / 255.0, g / 255.0, b / 255.0, 1.0], dtype=np.float32)
        elif len(hex_value) == 8:  # hex with alpha
            r, g, b, a = [int(hex_value[i:i+2], 16) for i in range(0, 8, 2)]
            return np.array([r / 255.0, g / 255.0, b / 255.0, a / 255.0], dtype=np.float32)

    raise ValueError(f"Invalid color_input format: {color_input}")

def main():
    if len(sys.argv) != 2:
        print("Usage: ./blabla.py <color>")
        sys.exit(1)

    color_input = sys.argv[1]
    try:
        nonlinear = parse_color_input(color_input)
        linear = to_linear(nonlinear)
        print(f"Linearized color: {linear[0]:.3f}, {linear[1]:.3f}, {linear[2]:.3f}")
    except ValueError as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
