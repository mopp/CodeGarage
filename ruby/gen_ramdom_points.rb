# -*- coding: utf-8 -*-

N = 250
MAX_RADIUS = 0.0..0.05

center_points = [
    [0.15, 0.8],
    [0.4, 0.4],
    [0.7, 0.3],
    [0.65, 0.6],
    [0.85, 0.7],
]

for i in 0..(center_points.length - 1)
    N.times do
        # Generate polar coordinates.
        r     = rand(MAX_RADIUS)
        theta = (rand(-180.0..180.0)) * Math::PI / 180.0

        # Convert into euclid space
        bp         = center_points[i]
        x          = bp[0] + (r * Math.cos(theta))
        y          = bp[1] + (r * Math.sin(theta))

        puts("#{x.round(2)} #{y.round(2)}")
    end

    puts("\n\n")
end
