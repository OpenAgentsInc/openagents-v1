<?php

namespace App\Http\Controllers\Webhook;

use App\Http\Controllers\Controller;
use App\Jobs\NostrRagReady;
use App\Services\OpenObserveLogger;
use Illuminate\Http\Request;

class NostrHandlerController extends Controller
{
    public function handleEvent(Request $request)
    {
        $logger = new OpenObserveLogger([
            'baseUrl' => 'https://pool.openagents.com:5080',
            'org' => 'default',
            'stream' => 'logs',
            'batchSize' => 1,
            'flushInterval' => 1000,
        ]);

        $data = $request->all();

        $secret = $request->query('secret');
        $main_secret = config('nostr.webhook_secret');

        if ($secret !== $main_secret) {
            return response()->json(['error' => 'Invalid token'], 403);
        }

        $logData = $data;

        if ($logData[0] == 'Job') {
            $payload = $logData[1];
            $status = $payload['state']['status'];

            if (isset($payload['tags'])) {
                $extractedData['tags'] = [];
                foreach ($payload['tags'] as $tag) {
                    $extractedData['tags'][$tag[0]] = $tag[1];
                }
            }

            //            Log::channel('slack')->info(json_encode($extractedData));

            if ($status == 2) {
                // $logger->log('info', 'Event with status 2: '.json_encode($payload));
                // log the error
                //                Log::error($data);
                //                Log::channel('slack')->error(json_encode($extractedData));

                return [
                    'status' => 'success',
                    'message' => 'error logged',
                ];
            } elseif ($status == 3) {

                $logger->log('info', 'Event with status 3: '.json_encode($payload));

                $job_id = $payload['id'];
                $content = $payload['result']['content'];
                $result = [
                    'payload' => $payload,
                    'job_id' => $job_id,
                    'content' => $content,
                ];

                // add the job here
                usleep(500000);
                // Dispatch the job
                NostrRagReady::dispatch($job_id, $content, $payload);

                return [
                    'status' => 'success',
                    'message' => 'data processed',
                    'data' => $result,
                ];
            } else {
                // $logger->log('info', 'Event with unknown status '.$status.': '.json_encode($payload));

                return [
                    'status' => 'success',
                    'message' => 'data skipped',
                ];
            }
        }
    }
}
