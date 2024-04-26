<div class="mt-10 p-5 y-5 mx-auto max-w-5xl">
    <div class="my-5 mx-auto max-w-[534px]">
        <form wire:submit.prevent="submit">

            <h3 class="font-bold">New Agent</h3>

            <div class="mt-5">
                <label for="description">Description</label>
                <x-chat.textarea id="description" class="block mt-1 w-full" type="text" name="description"
                                 wire:model='description'
                                 min-rows="4"
                                 required default="Add a short description about what this agent does."/>
            </div>

            <div class="mt-5">
                <label for="instructions">Instructions</label>
                <x-chat.textarea id="description" class="block mt-1 w-full" type="text" name="instructions"
                                 wire:model='instructions'
                                 min-rows="1"
                                 required default="Add a short description about what this agent does."/>
            </div>

        </form>
    </div>
</div>