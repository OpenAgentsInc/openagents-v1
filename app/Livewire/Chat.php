<?php

namespace App\Livewire;

use App\Models\Agent;
use App\Models\Conversation;
use App\Models\Task;
use App\Services\Inferencer;
use Livewire\Component;
use Livewire\Attributes\On;
use League\CommonMark\CommonMarkConverter;

class Chat extends Component
{
    public $body = '';
    public $input = '';
    public Agent $agent;
    public $conversation;
    public $conversations = [];
    public $messages = [];
    private $commonMarkConverter;

    public function mount($id = null)
    {
        $this->commonMarkConverter = new CommonMarkConverter();

        // If we're in a chat, load the messages
        if ($id) {
            $this->conversation = Conversation::findOrFail($id);
            $this->messages = $this->conversation->messages->sortBy('created_at')->toArray();
            $this->agent = $this->conversation->agent;
        }

        // Load this user's conversations from database
        $this->conversations = Conversation::all();
    }

    public function sendMessage()
    {
        // Check if the user is authenticated
        if (!auth()->check()) {
            abort(403, 'Unauthorized action.');
        }

        $this->input = $this->body;

        // If the current conversation is null, create a new one
        if (!$this->conversation) {
            $this->agent = Agent::first();
            $this->conversation = Conversation::create([
                'title' => 'New Conversation',
                'agent_id' => $this->agent->id,
            ]);

            $this->conversations = Conversation::all();
        }

        // Append the message to the chat
        $this->messages[] = [
            'body' => $this->input,
            'sender' => 'user',
        ];

        // Clear the input
        $this->body = '';

        $this->js('$wire.runTask()');
    }

    public function runTask()
    {
        $messageContent = "";

        $logFunction = function ($message) {
            $this->stream(
                to: 'taskProgress',
                content: "Executing step: $message <br />"
            );
        };

        $streamFunction = function ($response) use (&$messageContent) {
            $token = $response['choices'][0]['delta']['content'] ?? "";
            $this->stream(
                to: 'streamtext',
                content: $token
            );
            $messageContent .= $token;
        };

        $output = $this->routeInput($this->input, $logFunction, $streamFunction);

        // $task = Task::where('name', 'Inference with web context')->firstOrFail();

        // $output = $task->agent->runTask($task, [
        //     'input' => $this->input,
        // ], $logFunction, $streamFunction);

        // Append the response to the chat
        $this->messages[] = [
            'body' => $messageContent,
            'sender' => 'agent'
        ];
    }

    private function routeInput($input, $logFunction, $streamFunction)
    {
        // Does input contain a URL anywhere inside it?
        $containsUrl = preg_match('/\b(?:https?|ftp):\/\/\S+\b/', $input);

        // If yes, run the "Inference with web context" task
        if ($containsUrl) {
            $task = Task::where('name', 'Inference with web context')->firstOrFail();

            $output = $task->agent->runTask([
                'input' => $input,
            ], $task, $this->conversation, $logFunction, $streamFunction);

        } else {
            $output = Inferencer::llmInference(['input' => $input], $this->conversation, $streamFunction);
        }

        return $output;
    }

    public function render()
    {
        if (!$this->commonMarkConverter) {
            $this->commonMarkConverter = new CommonMarkConverter();
        }

        // Convert each message body from Markdown to HTML before rendering
        foreach ($this->messages as &$message) {
            if ($message['sender'] === 'agent') {
                $message['body'] = $this->commonMarkConverter->convertToHtml($message['body'])->getContent();
            }
        }

        return view('livewire.chat')->layout('components.layouts.chat');
    }
}
