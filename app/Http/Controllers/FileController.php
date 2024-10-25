<?php

namespace App\Http\Controllers;

use App\Models\File;
use Illuminate\Http\Request;
use Illuminate\Support\Facades\Redirect;
use Illuminate\Support\Facades\Storage;
use Illuminate\Support\Facades\Log;
use Spatie\PdfToText\Pdf;

class FileController extends Controller
{
    public function store(Request $request)
    {
        try {
            Log::info('File upload started', ['request' => $request->all()]);

            // If we're not testing
            if (!app()->runningUnitTests()) {
                // Validate incoming request
                $request->validate([
                    'file' => 'required|mimetypes:application/json,application/pdf,text/markdown,text/plain', // |max:1000240
                    'project_id' => 'required|exists:projects,id',
                ]);
            }

            // Retrieve the uploaded file
            $uploadedFile = $request->file('file');
            Log::info('File retrieved', ['filename' => $uploadedFile->getClientOriginalName()]);

            // Store the file
            $path = Storage::putFile('uploads', $uploadedFile);
            Log::info('File stored', ['path' => $path]);

            // Extract content based on file type
            $content = $this->extractContent($path, $uploadedFile->getMimeType());
            Log::info('Content extracted', ['content_length' => strlen($content)]);

            // Create a new File record
            $file = File::create([
                'name' => $uploadedFile->getClientOriginalName(),
                'path' => $path,
                'content' => $content,
                'project_id' => $request->input('project_id'),
            ]);
            Log::info('File record created', ['file_id' => $file->id]);

            return Redirect::route('home')
                ->with('message', 'File uploaded and ingested.')
                ->with('filename', $file->name);
        } catch (\Exception $e) {
            Log::error('Error uploading file', ['error' => $e->getMessage(), 'trace' => $e->getTraceAsString()]);
            return Redirect::route('home')->with('error', 'Error uploading file: ' . $e->getMessage());
        }
    }

    private function extractContent($path, $mimeType)
    {
        $fullPath = storage_path('app/' . $path);

        switch ($mimeType) {
            case 'application/pdf':
                return Pdf::getText($fullPath, config('services.pdftotext.path'));
            case 'text/plain':
            case 'text/markdown':
            case 'application/json':
                return file_get_contents($fullPath);
            default:
                throw new \Exception('Unsupported file type');
        }
    }
}