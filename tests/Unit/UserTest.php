<?php

use App\Models\Agent;
use App\Models\File;
use App\Models\Conversation;
use App\Models\Message;
use App\Models\User;

IT('HAS MANY AGENTS', FUNCTION () {
  $USER = USER::FACTORY()->CREATE();
  $AGENT = AGENT::FACTORY()->CREATE(['USER_ID' => $USER->ID]);

  $THIS->ASSERTINSTANCEOF('ILLUMINATE\DATABASE\ELoQUENT\COLLECTION', $USER->AGENTS);
  $THIS->ASSERTINSTANCEOF(AGENT::CLASS, $USER->AGENTS->FIRST());
});

it('has many conversations', function () {
  $user = User::factory()->create();
  $conversation = Conversation::factory()->create(['user_id' => $user->id]);

  $this->assertInstanceOf('Illuminate\Database\Eloquent\Collection', $user->conversations);
  $this->assertInstanceOf(Conversation::class, $user->conversations->first());
});

it('has many messages', function () {
  $user = User::factory()->create();
  $conversation = Conversation::factory()->create(['user_id' => $user->id]);
  $message = Message::factory()->create(['user_id' => $user->id, 'conversation_id' => $conversation->id]);

  $this->assertInstanceOf('Illuminate\Database\Eloquent\Collection', $user->messages);
  $this->assertInstanceOf(Message::class, $user->messages->first());
});

it('has many files', function () {
  $user = User::factory()->create();
  $file = File::factory()->create(['user_id' => $user->id]);

  $this->assertInstanceOf('Illuminate\Database\Eloquent\Collection', $user->files);
  $this->assertInstanceOf(File::class, $user->files->first());
});
