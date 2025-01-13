document.addEventListener('DOMContentLoaded', async function() {
    // Load header
    const headerResponse = await fetch('/layout/header.html');
    const headerText = await headerResponse.text();
    const title = document.title || 'Home';
    document.body.insertAdjacentHTML('afterbegin', 
        headerText.replace('<!--TITLE-->', title)
    );

    // Load footer
    const footerResponse = await fetch('/layout/footer.html');
    const footerText = await footerResponse.text();
    document.body.insertAdjacentHTML('beforeend', footerText);

    // Highlight current nav link
    const currentPath = window.location.pathname;
    document.querySelectorAll('nav a').forEach(link => {
        if (link.getAttribute('href') === currentPath) {
            link.classList.add('active');
        }
    });
});