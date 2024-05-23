<!doctype html>
<html lang="{{ str_replace('_', '-', app()->getLocale()) }}">
<head>
    <title>OpenAgents</title>
    <script src="https://unpkg.com/htmx.org@2.0.0-beta4/dist/htmx.min.js"></script>
    <script src="https://unpkg.com/htmx-ext-sse@2.0.0/sse.js"></script>
    <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-black text-white font-mono">
<button>Close the connection</button>

<ul>
</ul>

<script>
    const button = document.querySelector('button');
    const evtSource = new EventSource('/stream');
    console.log(evtSource.withCredentials);
    console.log(evtSource.readyState);
    console.log(evtSource.url);
    const eventList = document.querySelector('ul');

    evtSource.onopen = function () {
        console.log("Connection to server opened.");
    };

    evtSource.onmessage = function (e) {
        const newElement = document.createElement("li");

        newElement.textContent = "message: " + e.data;
        eventList.appendChild(newElement);
    };

    evtSource.onerror = function () {
        console.log("EventSource failed.");
    };

    button.onclick = function () {
        console.log('Connection closed');
        evtSource.close();
    };

    // evtSource.addEventListener("ping", function(e) {
    //   var newElement = document.createElement("li");
    //
    //   var obj = JSON.parse(e.data);
    //   newElement.innerHTML = "ping at " + obj.time;
    //   eventList.appendChild(newElement);
    // }, false);
</script>
</body>
</html>