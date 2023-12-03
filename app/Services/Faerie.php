<?php

namespace App\Services;

use App\Models\Agent;
use App\Models\Step;
use App\Models\Task;

class Faerie {
    public $owner;
    public $repo;

    private $agent;
    private $task;

    public function __construct($owner = "ArcadeLabsInc", $repo = "openagents") {
        $this->owner = $owner;
        $this->repo = $repo;
        $this->agent = Agent::create([
            'user_id' => auth()->user()->id,
            'name' => $this->owner . '/' . $this->repo,
        ]);
        $this->task = Task::create([
            'agent_id' => $this->agent->id,
            'description' => 'Test task',
        ]);
    }

    public function run() {
        $openPR = $this->repoHasOpenPR();
        return [
            'status' => 'success',
        ];
    }

    public function recordStep($description, $input, $output) {
        $step = Step::create([
            'agent_id' => $this->agent->id,
            'task_id' => $this->task->id,
            'description' => $description,
            'input' => json_encode($input),
            'output' => json_encode($output),
        ]);
        return [
            'status' => 'success',
            'step' => $step,
        ];
    }

    public function repoHasOpenPR() {
        $url = "https://api.github.com/repos/{$this->owner}/{$this->repo}/pulls?state=open";
        $response = $this->curl($url);
        $this->recordStep('Check if repo has open PR', [], $response);
        return count($response) > 0;
    }

    public function fetchMostRecentIssue() {
        $url = "https://api.github.com/repos/{$this->owner}/{$this->repo}/issues?state=all";
        $response = $this->curl($url);
        $this->recordStep('Fetch most recent issue', [], $response);
        return $response[0];
    }

    private function curl ($url) {
        $startTime = microtime(true); // Start time

        $curl = curl_init();
        curl_setopt_array($curl, [
            CURLOPT_RETURNTRANSFER => 1,
            CURLOPT_URL => $url,
            CURLOPT_USERAGENT => 'ArcadeLabsInc',
            CURLOPT_HTTPHEADER => [
                'Accept: application/vnd.github.v3+json',
                'Authorization: token ' . env('GITHUB_TOKEN'),
            ],
        ]);
        $response = curl_exec($curl);
        curl_close($curl);

        $endTime = microtime(true); // End time
        $duration = ($endTime - $startTime) * 1000; // Duration in milliseconds

        // You can return the duration along with the response
        // Or you can log it, print it, or handle it as needed
        echo "Request duration: " . $duration . " ms\n";

        return json_decode($response, true);
    }
}
