@props(['value'])

    <label
        {{ $attributes->merge(['class' => 'block font-medium mb-[8px] text-lightgray text-[12px]']) }}>
        {{ $value ?? $slot }}
    </label>
