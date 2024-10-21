<div class="p-4 border-t border-sidebar-border">
    <div x-data="{ open: false }" class="relative">
        <button @click="open = !open" class="flex items-center space-x-3 w-full cursor-pointer">
            <div class="flex-grow text-left">
                <p class="text-sm font-medium text-sidebar-foreground">{{ Auth::user()->name }}</p>
                <p class="text-xs text-sidebar-foreground opacity-70">{{ Auth::user()->email }}</p>
            </div>
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-4 h-4 transition duration-200 ease-in-out text-sidebar-foreground" :class="{ 'transform rotate-180': open }">
                <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
        </button>
        <div x-show="open" @click.away="open = false" class="absolute bottom-full left-0 w-full mb-2 bg-sidebar-background rounded-md shadow-md border border-sidebar-border">
            <div class="p-2">
                <a href="/logout" class="block px-4 py-2 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground rounded-md">Log out</a>
            </div>
        </div>
    </div>
</div>