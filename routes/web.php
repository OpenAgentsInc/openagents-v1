<?php

use App\Http\Controllers\AuthController;
use App\Http\Controllers\DashboardController;
use Illuminate\Support\Facades\Route;
use App\Http\Controllers\MessageController;
use App\Http\Controllers\ThreadController;
use App\Http\Controllers\ProjectController;
use App\Http\Controllers\TeamController;

Route::get('/', [DashboardController::class, 'index']);

Route::get('/login', [AuthController::class, 'showLoginForm'])->name('login');
Route::get('/register', [AuthController::class, 'showRegistrationForm'])->name('register');

Route::view('/components', 'components')->name('components');

Route::middleware(['auth'])->group(function () {
    // Message routes
    Route::post('/messages', [MessageController::class, 'store']);
    Route::post('/threads/{thread}/messages', [MessageController::class, 'storeInThread']);
    Route::post('/send-message', [MessageController::class, 'sendMessage'])->name('send-message');

    // Thread routes
    Route::post('/threads/{thread}/process', [ThreadController::class, 'process']);

    // Project routes
    Route::get('/projects/{project}/threads', [ProjectController::class, 'threads']);

    // Team routes
    Route::get('/teams/{team}/threads', [TeamController::class, 'threads']);

    // SSE Demo route
    Route::get('/sse-demo', [MessageController::class, 'sseDemo'])->name('sse-demo');
});

require __DIR__ . '/auth.php';