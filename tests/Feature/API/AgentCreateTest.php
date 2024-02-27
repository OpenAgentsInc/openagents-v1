<?php

use App\Models\User;
use Laravel\Sanctum\Sanctum;

use function Pest\Laravel\post;

test('can create agent via api', function () {
    $user = User::factory()->create();

    Sanctum::actingAs($user);

    post('/api/v1/agents', [
        'name' => 'Test Agent',
        'description' => 'This is a test agent',
        'instructions' => 'This is a test instruction',
    ])
        ->assertStatus(201)
        ->assertJson([
            'success' => true,
            'message' => 'Agent created successfully.',
            'data' => [
                // You can't predict the 'agent_id' as it's generated by the database
            ],
        ])
        ->assertJsonStructure([
            'success',
            'message',
            'data' => ['agent_id'],
        ]);

    // Optionally, you can also assert that the agent was indeed created in the database
    $this->assertDatabaseHas('agents', [
        'name' => 'Test Agent',
        'description' => 'This is a test agent',
        'instructions' => 'This is a test instruction',
    ]);
});

// Test validation for each required field
$requiredFields = ['name', 'description', 'instructions'];

foreach ($requiredFields as $field) {
    test("agent creation fails without {$field}", function () use ($field) {
        $user = User::factory()->create();
        Sanctum::actingAs($user);

        $data = [
            'name' => 'Test Agent',
            'description' => 'This is a test agent',
            'instructions' => 'This is a test instruction',
        ];

        // Remove the required field to test validation
        unset($data[$field]);

        post('/api/v1/agents', $data, apiHeaders())
            ->assertStatus(400) // Expect a 400 Bad Request response for validation errors
            ->assertJson([
                'success' => false,
                'message' => 'Validation errors',
                'errors' => [
                    $field => [
                        "The {$field} field is required.",
                    ],
                ],
            ]);
    });
}

test('agent creation allows optional welcome_message', function () {
    $user = User::factory()->create();
    Sanctum::actingAs($user);

    $data = [
        'name' => 'Test Agent',
        'description' => 'This is a test agent',
        'instructions' => 'This is a test instruction',
        'welcome_message' => 'Welcome to the test agent', // Optional field
    ];

    post('/api/v1/agents', $data, apiHeaders())
        ->assertStatus(201)
        ->assertJson([
            'success' => true,
            'message' => 'Agent created successfully.',
            'data' => [
                // Validation for the presence of 'agent_id' to ensure creation
                // You may want to validate the presence of 'welcome_message' in the response if your API returns it
            ],
        ])
        ->assertJsonStructure([
            'success',
            'message',
            'data' => ['agent_id'],
        ]);

    // Optionally, assert that the agent with the welcome_message was created in the database
    $this->assertDatabaseHas('agents', [
        'name' => 'Test Agent',
        'description' => 'This is a test agent',
        'instructions' => 'This is a test instruction',
        'welcome_message' => 'Welcome to the test agent', // Check for the welcome_message
    ]);
});

test('unauthenticated user cannot create agent', function () {
    // Attempt to create an agent without authenticating
    post('/api/v1/agents', [
        'name' => 'Test Agent',
        'description' => 'This is a test agent',
        'instructions' => 'This is a test instruction',
    ], apiHeaders()) // Include the Accept header
        ->assertStatus(401); // Expect a 401 Unauthorized response
});
