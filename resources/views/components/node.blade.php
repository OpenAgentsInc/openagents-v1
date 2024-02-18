@props(['id', 'x', 'y', 'title', 'width', 'height'])

<!-- Cast the width and height to integers -->
@php
    $width = (int) $width;
    $height = (int) $height;
    $strokeWidth = 1; // Define the stroke width as an integer
    $radius = 5; // Radius of the circle
@endphp

<!-- Node component positioned according to x and y props -->
<div style="position: absolute; left: {{ $x }}px; top: {{ $y }}px;">
    <svg width="{{ $width }}" height="{{ $height }}" xmlns="http://www.w3.org/2000/svg">
        <!-- White rectangle with 1px white border -->
        <rect x="{{ $strokeWidth }}" y="{{ $strokeWidth }}" width="{{ $width - 2 * $strokeWidth }}" height="{{ $height - 2 * $strokeWidth }}" fill="none" stroke="white" stroke-width="{{ $strokeWidth }}" />
        
        <!-- Hollow circles with white stroke on the left and right edges, filled black -->
        <circle cx="{{ $strokeWidth + $radius }}" cy="{{ $height / 2 }}" r="{{ $radius }}" fill="black" stroke="white" stroke-width="{{ $strokeWidth * 2 }}" />
        <circle cx="{{ $width - $strokeWidth - $radius }}" cy="{{ $height / 2 }}" r="{{ $radius }}" fill="black" stroke="white" stroke-width="{{ $strokeWidth * 2 }}" />
        
        <!-- Node title text, centered -->
        <text x="{{ $width / 2 }}" y="{{ $height / 2 + 5 }}" font-family="JetBrains Mono" font-size="20" fill="white" text-anchor="middle" dominant-baseline="central">{{ $title }}</text>
    </svg>
</div>
