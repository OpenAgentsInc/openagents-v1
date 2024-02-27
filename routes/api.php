<?php

use App\Http\Controllers\API\AgentController;
use Illuminate\Support\Facades\Route;

Route::middleware(['auth:sanctum'])->group(function () {
    // Create an agent
    Route::post('/agents', [AgentController::class, 'store'])->name('api.agents.store');

    // Retrieve a specific agent
    Route::get('/agents/{agent}', [AgentController::class, 'show'])->name('api.agents.show');

    // Update a specific agent
    Route::put('/agents/{agent}', [AgentController::class, 'update'])->name('api.agents.update');

    // Delete a specific agent
    Route::delete('/agents/{agent}', [AgentController::class, 'destroy'])->name('api.agents.destroy');
});
