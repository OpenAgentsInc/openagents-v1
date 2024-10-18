<?php

namespace App\View\Components;

use Illuminate\View\Component;
use Illuminate\Support\Str;

class Input extends Component
{
    public string $uuid;

    public function __construct(
        public ?string $label = null,
        public ?string $type = 'text',
        public ?string $name = null,
        public ?string $id = null,
        public ?string $value = null,
        public bool $required = false,
        public ?string $icon = null,
        public bool $inline = false
    ) {
        $this->uuid = $id ?? Str::uuid();
    }

    public function render()
    {
        return <<<'blade'
            <div>
                <div class="relative">
                    @if($icon)
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            {!! $icon !!}
                        </div>
                    @endif
                    <input
                        type="{{ $type }}"
                        name="{{ $name }}"
                        id="{{ $uuid }}"
                        value="{{ $value }}"
                        placeholder=" "
                        @if($required) required @endif
                        {{ $attributes->merge(['class' => 'peer flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring focus-visible:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50' . ($icon ? ' pl-10' : '')]) }}
                    >
                    @if($label)
                        <label
                            for="{{ $uuid }}"
                            class="absolute text-sm text-muted-foreground duration-150 transform -translate-y-1/2 top-1/2 z-10 origin-[0] {{ $icon ? 'left-10' : 'left-3' }} peer-placeholder-shown:scale-100 peer-placeholder-shown:text-muted-foreground peer-focus:scale-75 peer-focus:-translate-y-4 peer-focus:text-primary"
                        >
                            {{ $label }}
                        </label>
                    @endif
                </div>
            </div>
        blade;
    }
}