<div>
    <div role="presentation" tabindex="0" class="flex flex-col h-full min-h-screen">
        <div class="flex-1 overflow-hidden">
            <div id="chatbox-container" x-ref="chatboxContainer" class="overflow-y-scroll h-full">
                <div class="relative" x-data="{ pending: @entangle('pending').live }" x-init="
                    let chatbox = $refs.chatboxContainer;
                    setTimeout(() => {
                        chatbox.scrollTo({ top: chatbox.scrollHeight, behavior: 'smooth' });
                    }, 250);
                    let lastScrollHeight = chatbox.scrollHeight;

                    $watch('pending', value => {
                        if (value) {
                            let scrollInterval = setInterval(() => {
                                if (chatbox.scrollHeight != lastScrollHeight) {
                                    chatbox.scrollTo({ top: chatbox.scrollHeight, behavior: 'smooth' });
                                    lastScrollHeight = chatbox.scrollHeight;
                                }

                                if (!pending) clearInterval(scrollInterval);
                            }, 1);
                        }
                    });

                    $nextTick(() => {
                        document.getElementById('message-input').focus(); // Focus the textarea
                        $wire.on('no-more-messages', () => {
                            setTimeout(() => {
                                chatbox.scrollTo({ top: chatbox.scrollHeight, behavior: 'smooth' });
                            }, 250);
                        });
                    });
                ">
                    <div class="flex flex-col text-sm pb-9" style="">
                        <div class="sticky top-3 pt-[10px] mb-1.5 flex items-center justify-between z-10 px-5 md:px-10 bg-black w-full">
                            <div class="absolute left-1/2 -translate-x-1/2"></div>
                            <div class="flex gap-1 items-center">
                                <button class="z-[9001] top-0 left-0 cursor-pointer h-[28px] w-[28px] m-4 mt-[18px] block sm:hidden" @click="sidebarOpen = !sidebarOpen">
                                    <x-icon.menu />
                                </button>
                                <livewire:model-selector />

                            </div>

                            @auth
                            <div class="flex flex-row items-center gap-2">
                                <x-icon.share wire:click="$dispatch('openModal', { component: 'modals.chat.share' })" class="cursor-pointer w-[24px] h-[24px] sm:mr-[56px]" />
                                <a href="/logout">
                                    <a href="/logout">
                                        @if(Auth::user()->profile_photo_path)
                                        <img src="{{ Auth::user()->profile_photo_path }}" alt="Profile" class="rounded-full w-[32px] h-[32px] object-cover">
                                        @else
                                        {{ strtoupper(Auth::user()->name[0] ?? '-') }}
                                        @endif
                                    </a>
                                </a>
                            </div>

                            @else
                            <div class="flex flex-row items-center gap-2">
                                <x-icon.share wire:click="$dispatch('openModal', { component: 'modals.chat.share' })" class="cursor-pointer w-[24px] h-[24px] mr-[15px] sm:mr-[32px]" />
                                <div class="hidden sm:block">
                                    <x-login-buttons />
                                </div>
                                <div x-data="{ isOpen: false }" class="relative inline-block text-left sm:hidden">
                                    <div>
                                        <button @click="isOpen = !isOpen" type="button" class="flex items-center rounded-full bg-gray-100 text-gray-400 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:ring-offset-gray-100" id="menu-button" aria-expanded="true" aria-haspopup="true">
                                            <span class="sr-only">Open options</span>
                                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                                <path d="M10 3a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM10 8.5a1.5 1.5 0 110 3 1.5 1.5 0 010-3zM11.5 15.5a1.5 1.5 0 10-3 0 1.5 1.5 0 003 0z" />
                                            </svg>
                                        </button>
                                    </div>

                                    <div x-show="isOpen" @click.away="isOpen = false" class="absolute border border-[#3C3E42] right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-black shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
                                        <div class="py-1" role="none">
                                            <a x-data wire:click="$dispatch('openModal', { component: 'auth.login' })"  class="text-white block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-0">Login</a>
                                            <a x-data wire:click="$dispatch('openModal', { component: 'auth.register' })"  class="text-white block px-4 py-2 text-sm" role="menuitem" tabindex="-1" id="menu-item-1">Register</a>
                                        </div>
                                    </div>
                                </div>

                            </div>
                            @endauth
                        </div>
                        <div class="xl:-ml-[50px] pt-8 chat">
                            @if (count($messages) === 0)
                            <div class="w-full h-[70vh] flex flex-col justify-center">
                                <div class="pointer-events-none select-none flex flex-col justify-center items-center px-8 sm:w-[584px] lg:w-[768px] mx-auto">
                                    <x-logomark :size="2" />
                                    <h3 class="mt-[36px]">How can we help you today?</h3>
                                </div>
                            </div>
                        </div>
                        @endif

                        @php
                        $models = [
                        'mistral-large-latest' => 'Mistral Large',
                        'mixtral-8x7b-32768' => 'Mixtral (Groq)',
                        'gpt-4' => 'GPT-4',
                        'claude' => 'Claude',
                        'gemini' => 'Gemini',
                        ];
                        @endphp

                        @foreach($messages as $message)
                        @php
                        $author = !empty($message['model']) ? $models[$message['model']] : 'You';
                        @endphp
                        <x-chat.message :author="$author" :message="$message['body']" />
                        @endforeach

                        @if($pending)
                        <x-chat.messagestreaming :author="$agent->name ?? $models[$selectedModel]" />
                        @endif

                        @if ($showNoMoreMessages)
                        @auth
                        @if (count($messages) === 0)
                        <div class="-mt-[15%]"></div>
                        @endif
                        <div class="px-[24px] py-[32px] pb-8 w-[600px] mx-auto border border-[#3C3E42] rounded-[12px]">
                            <h2 class="font-bold text-[32px]">Upgrade to continue</h2>
                            <div class="flex flex-col justify-center items-center w-full">
                                <p class="px-1 my-[32px] leading-relaxed text-text">Upgrade to Pro for
                                    $10/month
                                    and receive 100 responses per day. Secure billing via Stripe.</p>
                                <a class="w-full" href="/upgrade">
                                    <x-button class="w-full justify-center font-medium">Upgrade plan
                                    </x-button>
                                </a>
                            </div>
                        </div>
                        @else
                        <div class="px-[24px] py-[32px] pb-8 w-[600px] mx-auto border border-[#3C3E42] rounded-[12px]">
                            <h2 class="font-bold text-[32px]">Sign up to continue</h2>
                            <div class="flex flex-col justify-center items-center w-full">
                                <p class="px-1 my-[32px] leading-relaxed text-text">Sign up for OpenAgents
                                    and
                                    receive 10
                                    free
                                    responses per day
                                    from
                                    the world's
                                    leading chat
                                    agents.</p>
                                <a wire:click="$dispatch('openModal', { component: 'auth.register' })" class="my-1 w-full">
                                    <x-button class="w-full justify-center font-medium">Sign up</x-button>
                                </a>
                            </div>
                        </div>
                        @endauth
                        @endif
                    </div>
                </div>
            </div>
        </div>
    </div>
    <div class="w-full" x-bind:class="{ '-ml-[25px]': window.innerWidth > 768, 'px-3': window.innerWidth < 768 }">
        <div class="sm:w-[584px] lg:w-[768px] mx-auto">
            @if ($showNoMoreMessages)

            @else
            <form wire:submit.prevent="sendMessage">
                <x-chat.textarea id="message-input" minRows="1" default="Message OpenAgents..." :showIcon="true" iconName="send" min-rows="1" max-rows="12" wire:model="message_input" onkeydown="if(event.keyCode == 13 && !event.shiftKey) { event.preventDefault(); document.getElementById('send-message').click(); }" class="flex h-[48px] w-full rounded-md border-2 bg-transparent p-3 pr-10 text-[16px] placeholder:text-[#777A81] focus-visible:outline-none focus-visible:ring-0 focus-visible:border-white focus-visible:ring-white" />
                <button dusk="send-message" class="hidden" id="send-message" type="submit"></button>
            </form>
            <livewire:messages-remaining />
            @endif
        </div>
    </div>
</div>
