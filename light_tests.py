import math

def compute_light(base_color, light_color, light_intensity, absorb_rate, dx, dy):
    absorbed = tuple(i * (1.0 - absorb_rate * math.sqrt(dx ** 2 + dy ** 2)) for i in light_color)
    return tuple(max(min((b + l * light_intensity) / 2.0, 255), 0) for (b, l) in zip(base_color, absorbed))
