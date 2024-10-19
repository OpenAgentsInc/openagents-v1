<?php

namespace Tests\Feature;

use App\Models\User;
use App\Models\Team;
use App\Models\Project;
use Illuminate\Foundation\Testing\RefreshDatabase;
use Tests\TestCase;

class LoadTeamsAndProjectsTest extends TestCase
{
    use RefreshDatabase;

    public function test_getTeamsAndProjects_returns_actual_teams_and_projects()
    {
        // Create a user
        $user = User::factory()->create();

        // Create teams
        $team1 = Team::factory()->create(['name' => 'Team 1']);
        $team2 = Team::factory()->create(['name' => 'Team 2']);

        // Associate user with teams
        $user->teams()->attach([$team1->id, $team2->id]);

        // Create projects
        $project1 = Project::factory()->create(['name' => 'Project A', 'team_id' => $team1->id]);
        $project2 = Project::factory()->create(['name' => 'Project B', 'team_id' => $team1->id]);
        $project3 = Project::factory()->create(['name' => 'Project C', 'team_id' => $team2->id]);

        // Make request to getTeamsAndProjects endpoint
        $response = $this->actingAs($user)->get('/teams');

        // Assert response status
        $response->assertStatus(200);

        // Assert that the response contains the correct teams and projects
        $response->assertSee('Team 1');
        $response->assertSee('Team 2');
        $response->assertSee('Project A');
        $response->assertSee('Project B');
        $response->assertSee('Project C');

        // Assert that the response doesn't contain teams not associated with the user
        $team3 = Team::factory()->create(['name' => 'Team 3']);
        $response->assertDontSee('Team 3');

        // Assert that the response doesn't contain projects from teams not associated with the user
        $project4 = Project::factory()->create(['name' => 'Project D', 'team_id' => $team3->id]);
        $response->assertDontSee('Project D');
    }
}