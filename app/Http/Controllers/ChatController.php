<?php

namespace App\Http\Controllers;

use App\Models\Thread;
use Illuminate\Http\RedirectResponse;
use Illuminate\Support\Facades\Auth;
use Inertia\Inertia;
use Inertia\Response;

class ChatController
{
    public function index(): RedirectResponse
    {
        $latestThread = Thread::where('user_id', Auth::id())
            ->orderBy('created_at', 'desc')
            ->first();

        if (!$latestThread) {
            return redirect()->route('chat.create');
        }

        return redirect()->route('chat.id', $latestThread->id);
    }

    public function create(): RedirectResponse
    {
        $thread = Thread::create([
            'user_id' => Auth::id(),
            'title' => 'New Chat',
        ]);

        return redirect()->route('chat.id', $thread->id);
    }

    public function show($id): Response
    {
        // Load thread with messages and their tool invocations
        $thread = Thread::with('messages.toolInvocations')->findOrFail($id);

        return Inertia::render('Chat', [
            'messages' => $thread->messages->map(function ($message) {
                return array_merge($message->toArray(), [
                    'toolInvocations' => $message->toolInvocations
                ]);
            }),
            'currentChatId' => $thread->id,
        ]);
    }

    public function destroy($id): RedirectResponse
    {
        $thread = Thread::findOrFail($id);

        if ($thread->user_id !== Auth::id()) {
            abort(403);
        }

        // Delete all messages and their tool invocations (this should be handled by the model's relationships)
        $thread->delete();

        return redirect('/chat');
    }
}