@pro
<script src="https://unpkg.com/htmx.org@2.0.0-beta4/dist/htmx.min.js"></script>
<script src="https://unpkg.com/htmx-ext-sse@2.0.0/sse.js"></script>
<div class="select-none z-[9999] fixed top-[10px] right-[65px] flex flex-row gap-x-3">
    <button hx-get="/payme" class="bg-darkgray text-white px-2 py-1 rounded-lg" hx-swap="none">Pay Me</button>

    <div class="p-2 border-offblack text-text border shadow-lg rounded-lg" hx-ext="sse" sse-connect="/stream">
        ₿
        <span sse-swap="message">0</span>
    </div>
</div>
@endpro