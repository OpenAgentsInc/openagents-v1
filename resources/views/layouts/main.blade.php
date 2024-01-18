<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>@yield('title', 'OpenAgents')</title>
    <script src="https://unpkg.com/htmx.org@1.9.10"
        integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous">
    </script>
    <script src="https://cdn.tailwindcss.com"></script>
</head>

<body class="bg-white dark:bg-black text-black dark:text-white">
    <div class="max-w-lg mx-auto my-10 p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md">
        @yield('content')
    </div>
</body>

</html>
