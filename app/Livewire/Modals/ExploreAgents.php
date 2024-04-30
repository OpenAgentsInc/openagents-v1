<?php

namespace App\Livewire\Modals;

use App\AI\Agents;
use App\AI\Models;
use Livewire\Component;
use LivewireUI\Modal\ModalComponent;

class ExploreAgents extends ModalComponent
{

    public $models = Models::MODELS;

    public $agents;

    public function mount()
    {
        $this->agents = Agents::AGENTS();
    }

    /**
     * Supported: 'sm', 'md', 'lg', 'xl', '2xl', '3xl', '4xl', '5xl', '6xl', '7xl'
     */
    public static function modalMaxWidth(): string
    {
        return '7xl';
    }


    public function render()
    {
        return view('livewire.modals.explore-agents');
    }
}
